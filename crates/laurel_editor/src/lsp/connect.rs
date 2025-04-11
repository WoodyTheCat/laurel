use futures::{SinkExt as _, Stream};
use iced::stream;
use tokio::sync::mpsc::{self, Receiver};
use tracing::{error, info};

use crate::lsp::{LspConnection, LspServerNotification, Synchronise};

use super::{client::LspClient, LspClientNotification, LspCommand, LspMessage, LspRequest};

/// How the internals of the stream are handled. This enum never leaves the stream and never is
/// touched by the gui. It is self managed state by the [`client`] stream.
#[derive(Debug, Default)]
enum State {
    Connected(
        LspClient,
        Receiver<LspCommand>,
        Receiver<LspClientNotification>,
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
                State::Connected(ref mut client, ref mut gui_receiver, ref mut notify_receiver) => {
                    tokio::select! {
                                        biased;
                                        // LspMessage -> LspRequest | LspServerNotification
                                        Some(msg) = gui_receiver.recv() => match msg {
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
                                                    Synchronise::DidClose => {let _ = client.did_close(uri).await;},
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
                                                }}
                                        },
                                                               // },
                                                               // LspMessage::Synchronise(synchronise) => match synchronise {
                                                               //     Synchronise::DidChange(string, path) => {
                                                               //         let _ = client.did_change(string, path).await;
                                                               //     }
                                                               //     Synchronise::DidClose => todo!(),
                                                               //     Synchronise::DidOpen(string, path) => {
                                                               //         let _ = client.did_open(string, path).await;
                                                               //     }
                                                               //     Synchronise::DidSave(string, path) => {
                                                               //         let _ = client.did_save(string, path);
                                                               //     }
                                                               //     Synchronise::WillSave(path) => {
                                                               //         let _ = client.will_save(path);
                                                               //     }
                                                           // LspClientNotification
                                                           Some(msg) = notify_receiver.recv() => {
                                                               info!(message = ?msg, "Notification from LSP");
                                                           }
                                                       };
                }
                State::Disconnected => {
                    let (lsp_sender, lsp_receiver) = mpsc::channel(CHANNEL_SIZE);
                    match LspClient::initialise(lsp_sender).await {
                        Ok(client) => {
                            info!("Successfully initialised LSP");
                            let (gui_sender, gui_receiver) = mpsc::channel(CHANNEL_SIZE);
                            let _ = output
                                .send(LspMessage::Initialised(LspConnection(gui_sender)))
                                .await;
                            state = State::Connected(client, gui_receiver, lsp_receiver);
                        }
                        Err(e) => {
                            error!(error = ?e, "Error whilst initialising the language server client");
                            let _ = output.send(LspMessage::Shutdown).await;
                        }
                    }
                }
            }
        }
    })
}
