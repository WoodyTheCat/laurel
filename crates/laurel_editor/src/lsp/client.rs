use std::{ops::ControlFlow, path::Path, process::Stdio};

use async_lsp::{
    concurrency::ConcurrencyLayer,
    lsp_types::{
        notification::{Progress, PublishDiagnostics, ShowMessage},
        ClientCapabilities, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, HoverContents, HoverParams,
        InitializeParams, InitializedParams, NumberOrString, Position, ProgressParamsValue,
        PublishDiagnosticsParams, Range, TextDocumentContentChangeEvent, TextDocumentIdentifier,
        TextDocumentItem, TextDocumentPositionParams, TextDocumentSaveReason, Url,
        VersionedTextDocumentIdentifier, WillSaveTextDocumentParams, WindowClientCapabilities,
        WorkDoneProgress, WorkDoneProgressParams,
    },
    panic::CatchUnwindLayer,
    router::Router,
    tracing::TracingLayer,
    LanguageServer as _, ServerSocket,
};
use tokio::{
    process::{Child, Command},
    spawn,
    sync::{mpsc::Sender, oneshot},
};
use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};
use tower::ServiceBuilder;
use tracing::{debug, error, info, trace, warn};

use crate::lsp::diagnostics::{ClientDiagnostics, Issue};

use super::{error::LspClientResult, LspClientNotification};

static LANGUAGE: &str = "rust";

struct ServerStop;

#[derive(Debug)]
pub struct LspClient {
    _process: Child,
    server: ServerSocket,
}

struct LspClientState {
    indexed_tx: Option<oneshot::Sender<()>>,
    lsp_sender: Sender<LspClientNotification>,
}

impl LspClient {
    pub async fn initialise(lsp_sender: Sender<LspClientNotification>) -> LspClientResult<Self> {
        // FIXME: Proper workspace recognition once the fs is in
        let root_dir = Path::new(".")
            .canonicalize()
            .expect("Workspace root should be valid");

        let (indexed_tx, indexed_rx) = oneshot::channel();

        let (mainloop, mut server) = async_lsp::MainLoop::new_client(move |_server| {
            let mut router = Router::new(LspClientState {
                lsp_sender,
                indexed_tx: Some(indexed_tx),
            });

            router
            .notification::<Progress>(|this, prog| {
                debug!("{:?} {:?}", prog.token, prog.value);
                // FIXME: Swap rustAnalyzer here for tinymist's name
                if matches!(prog.token, NumberOrString::String(s) if s == "rustAnalyzer/cachePriming")
                    && matches!(
                        prog.value,
                        ProgressParamsValue::WorkDone(WorkDoneProgress::End(_))
                    )
                {
                    // Sometimes rust-analyzer auto-index multiple times?
                    if let Some(tx) = this.indexed_tx.take() {
                        let _: Result<_, _> = tx.send(());
                            info!("Sent indexing completion oneshot");
                    } else {
                            error!("Indexing completion oneshot already sent");
                        }
                }
                ControlFlow::Continue(())
            })
            .notification::<PublishDiagnostics>(|st: &mut LspClientState, params: PublishDiagnosticsParams| {

                let issues: Vec<Issue> = params.diagnostics.into_iter().map(Issue::from).collect();
                    info!(issues = ?issues, "Issues notification");
                    // TODO: See if this can be made non-blocking
                    st.lsp_sender.blocking_send(
                LspClientNotification::Diagnostics(ClientDiagnostics {
                    issues,
                    uri: params.uri,
                })).unwrap();


                    ControlFlow::Continue(())})

            .notification::<ShowMessage>(|_, params| {
                info!("Message {:?}: {}", params.typ, params.message);
                ControlFlow::Continue(())
            })
                    .unhandled_notification(|_, a|
            {warn!(a = ?a, "Unhandled language server notification"); ControlFlow::Continue(())})
            .event(|_, _: ServerStop| ControlFlow::Break(Ok(())));

            ServiceBuilder::new()
                .layer(TracingLayer::default())
                .layer(CatchUnwindLayer::default())
                .layer(ConcurrencyLayer::default())
                .service(router)
        });

        let mut process = Command::new("rust-analyzer")
            .current_dir(&root_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .expect("Failed to run rust-analyzer");

        let stdout = process.stdout.take().unwrap().compat();
        let stdin = process.stdin.take().unwrap().compat_write();

        let _mainloop_future =
            spawn(async move { mainloop.run_buffered(stdout, stdin).await.unwrap() });

        info!("If this shows, you should be confused");

        // Initialize.
        let root_uri = Url::from_file_path(&root_dir).unwrap();
        let init_ret = server
            .initialize(InitializeParams {
                root_uri: Some(root_uri),
                capabilities: ClientCapabilities {
                    window: Some(WindowClientCapabilities {
                        work_done_progress: Some(true),
                        ..WindowClientCapabilities::default()
                    }),
                    ..ClientCapabilities::default()
                },
                ..InitializeParams::default()
            })
            .await
            .unwrap();

        info!("Initialized: {init_ret:?}");
        server.initialized(InitializedParams {}).unwrap();

        info!("Waiting for the LSP to index");

        // Wait until indexed.
        indexed_rx.await.unwrap();

        info!("LSP indexed");

        Ok(Self {
            _process: process,
            server,
        })
    }

    /// Shutdowns server, this method is not run on process termination.
    /// Only on user requested an explicit termination.
    pub async fn shutdown(&mut self) -> LspClientResult<()> {
        info!("LSP Client shutting down");
        self.server.shutdown(()).await.unwrap();
        self.server.exit(()).unwrap();
        self.server.emit(ServerStop).unwrap();
        trace!("LSP Client shutdown complete");
        Ok(())
    }

    pub async fn did_change(
        &mut self,
        uri: Url,
        range: Range,
        changed_to: String,
    ) -> LspClientResult<()> {
        trace!("didchange");

        self.server.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri,
                // TODO: Increase the version number before we do this
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                // OPTIMIZATION we need to send only the sections that have changed.
                // range is the starting..ending lines that have changed.
                // text is the text from the sections that have changed.
                // range_length is deprecated, keep as none.
                range: Some(range),
                range_length: None,
                text: changed_to,
            }],
        });

        trace!("didchanged");

        Ok(())
    }

    pub async fn did_open(&mut self, uri: Url, text: String) -> LspClientResult<()> {
        trace!("didopen");

        self.server
            .did_open(DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri,
                    language_id: LANGUAGE.into(),
                    version: 0,
                    text,
                },
            })
            .unwrap();

        Ok(())
    }

    pub async fn did_save(&mut self, uri: Url, text: Option<String>) -> LspClientResult<()> {
        self.server.did_save(DidSaveTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
            text,
        });

        Ok(())
    }

    pub async fn did_close(&mut self, uri: Url) -> LspClientResult<()> {
        self.server.did_close(DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        });

        Ok(())
    }

    pub async fn will_save(&mut self, uri: Url) -> LspClientResult<()> {
        self.server.will_save(WillSaveTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
            reason: TextDocumentSaveReason::MANUAL,
        });

        Ok(())
    }

    pub async fn hover(
        &mut self,
        uri: Url,
        position: Position,
    ) -> LspClientResult<Option<HoverContents>> {
        let hover = self
            .server
            .hover(HoverParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position,
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .await
            .unwrap();

        Ok(hover.map(|h| h.contents))
    }
}
