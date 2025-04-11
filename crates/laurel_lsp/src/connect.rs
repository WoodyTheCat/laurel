use futures::{SinkExt as _, Stream, channel::mpsc::Sender as FuturesSender};
use iced::stream;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tracing::{error, info};

use crate::{LspConnection, LspServerNotification, Synchronise};

use super::{LspClientNotification, LspCommand, LspMessage, LspRequest, client::LspClient};

/// How the internals of the stream are handled. This enum never leaves the stream and never is
/// touched by the gui. It is self managed state by the [`client`] stream.
#[derive(Debug, Default)]
enum State {
    Connected(
        LspClient,
        UnboundedReceiver<LspCommand>,
        UnboundedReceiver<LspClientNotification>,
    ),
    #[default]
    Disconnected,
}

pub fn connect() -> impl Stream<Item = LspMessage> {
    const CHANNEL_SIZE: usize = 1024;
    stream::channel(CHANNEL_SIZE, async |mut output| {
        let mut state = State::default();

        loop {
            match state {
                State::Connected(ref mut client, ref mut gui_rcv, ref mut lsp_rcv) => {
                    receive_message(client, &mut output, gui_rcv, lsp_rcv).await
                }
                State::Disconnected => {
                    let (lsp_sender, lsp_receiver) = mpsc::unbounded_channel();
                    match LspClient::initialize(lsp_sender).await {
                        Ok(client) => {
                            info!("Successfully initialized LSP");
                            let (gui_sender, gui_receiver) =
                                mpsc::unbounded_channel::<LspCommand>();
                            let _ = output
                                .send(LspMessage::Initialized(LspConnection(gui_sender)))
                                .await;
                            state = State::Connected(client, gui_receiver, lsp_receiver);
                        }
                        Err(e) => {
                            error!(error = ?e, "Error whilst initializing the language server client");
                            let _ = output.send(LspMessage::Shutdown).await;
                        }
                    }
                }
            }
        }
    })
}

async fn receive_message(
    client: &mut LspClient,
    output: &mut FuturesSender<LspMessage>,
    gui_rcv: &mut UnboundedReceiver<LspCommand>,
    lsp_rcv: &mut UnboundedReceiver<LspClientNotification>,
) {
    tokio::select! {
        biased;
        // LspCommand -> LspRequest | LspServerNotification
        Some(msg) = gui_rcv.recv() => match msg {
            LspCommand::Request(r) => match r {
                LspRequest::Shutdown => {
                    let _ = client.shutdown().await;
                }
            },
            LspCommand::Notification(n) => match n {
                LspServerNotification::Synchronise(s, uri) => {
                    match s {
                        Synchronise::DidChange(string, range) => {
                            let _ = client.did_change(uri, range.into(), string).await;
                        },
                        Synchronise::DidClose => {
                            let _ = client.did_close(uri).await;
                        },
                        Synchronise::DidOpen(string) => {
                            let _ = client.did_open(uri, string).await;
                        }
                        Synchronise::DidSave(string) => {
                            let _ = client.did_save(uri, string).await;
                        },
                        Synchronise::WillSave => {
                            let _ = client.will_save(uri).await;
                        }
                    }
                }
            }
        },
        // LspMessage
        Some(notification) = lsp_rcv.recv() => {
            let _ = output.send(LspMessage::Notification(notification)).await;
        }
    };
}
