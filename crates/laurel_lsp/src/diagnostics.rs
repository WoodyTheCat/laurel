use async_lsp::lsp_types::{self, DiagnosticSeverity, Url};
use iced::Color;

use laurel_common::text::{Position, Range};

#[derive(Debug, Clone)]
pub struct ClientDiagnostics {
    pub issues: Vec<Issue>,
    pub uri: Url,
}

impl ClientDiagnostics {
    pub fn diagnostic_in_position(&self, position: Position) -> Option<Issue> {
        self.issues
            .clone()
            .into_iter()
            .find(|value| value.range.pos_in_range(position))
    }
}

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

impl Severity {
    pub fn color(&self) -> Color {
        match self {
            Severity::Error => Color::from_rgba8(239, 48, 84, 0.9),
            Severity::Warning => Color::from_rgba8(245, 230, 99, 0.9),
            Severity::Info => Color::from_rgba8(71, 168, 189, 0.9),
            Severity::Hint => Color::from_rgba8(71, 168, 189, 0.9),
        }
    }
}

impl From<DiagnosticSeverity> for Severity {
    fn from(value: DiagnosticSeverity) -> Self {
        match value {
            DiagnosticSeverity::ERROR => Self::Error,
            DiagnosticSeverity::WARNING => Self::Warning,
            DiagnosticSeverity::INFORMATION => Self::Info,
            DiagnosticSeverity::HINT => Self::Hint,
            _ => panic!("Unknown Severity"),
        }
    }
}

/**
 * A warning or error from the LSP server.
 */
#[derive(Debug, Clone)]
pub struct Issue {
    pub range: Range,
    pub code_description: Option<lsp_types::CodeDescription>,
    pub message: String,
    pub severity: Severity,
}

impl From<lsp_types::Diagnostic> for Issue {
    fn from(value: lsp_types::Diagnostic) -> Self {
        let serverity = value.severity.unwrap_or(DiagnosticSeverity::ERROR);
        let severity = Severity::from(serverity);

        Self {
            range: Range::from(value.range),
            code_description: value.code_description,
            message: value.message,
            severity,
        }
    }
}
