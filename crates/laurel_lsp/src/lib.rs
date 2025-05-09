use std::{fs, path::PathBuf};

use async_lsp::lsp_types::Url;
pub use diagnostics::{ClientDiagnostics, Issue, Severity};
use tokio::sync::mpsc::{self};

use laurel_common::text::Range;

pub use connect::connect;

pub mod client;
pub mod connect;
pub mod diagnostics;
pub mod error;

#[derive(Debug, Clone)]
pub struct LspConnection(mpsc::UnboundedSender<LspCommand>);

impl LspConnection {
    pub fn send(&mut self, item: LspCommand) {
        // FIXME: This will crash the program if the stream has to reinit
        self.0.send(item).unwrap()
    }
}

/// A message from the Stream to the gui
#[derive(Debug, Clone)]
pub enum LspMessage {
    Initialized(LspConnection),
    Shutdown,

    Response(LspResponse),
    Notification(LspClientNotification),
}

/// A message from the gui to the Stream
#[derive(Debug, Clone)]
pub enum LspCommand {
    Request(LspRequest),
    Notification(LspServerNotification),
}

/// A notification sent from the server to the client
#[derive(Debug, Clone, Default)]
pub enum LspClientNotification {
    Diagnostics(ClientDiagnostics),
    Progress,
    ErrorMessage(String),
    Initialized,

    #[default]
    UnknownMessage,
}

/// A notification sent from the client to the server
#[derive(Debug, Clone)]
pub enum LspServerNotification {
    Synchronise(Synchronise, Url),
}

/// A request from the client to the server that requires a response
#[derive(Debug, Clone)]
pub enum LspRequest {
    Shutdown,
}

/// The response to an [`LspRequest`]
#[derive(Debug, Clone)]
pub enum LspResponse {
    None,
}

//

/// Messages for document synchronisation
#[derive(Debug, Clone)]
pub enum Synchronise {
    DidChange(String, Range),
    DidClose,
    DidOpen(String),
    DidSave(Option<String>),
    WillSave,
}

pub fn file_path(relative_path: &str) -> String {
    let path = PathBuf::from(relative_path);
    let absolute_path = fs::canonicalize(path).unwrap();

    format!("file://{}", absolute_path.to_str().unwrap())
}
