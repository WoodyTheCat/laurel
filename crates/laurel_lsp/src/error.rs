#[derive(Debug, Clone, thiserror::Error)]
pub enum LspClientError {
    #[error("Failed Initialization: `{0}`")]
    FailedInitialization(String),
    #[error("Process Failure: `{0}`")]
    ProcessFailure(String),
    #[error("Process Failure: `{0}`")]
    ChannelClosed(String),
}

pub type LspClientResult<T> = Result<T, LspClientError>;
