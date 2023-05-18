use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MrDamianError {
    // TODO: consider about error category and boundary.

    // ------- domain level errors
    #[error("message key not found")]
    MessageKeyNotFound,

    #[error("port not found")]
    PortNotFound(String),

    #[error("invalid component")]
    InvalidComponent,

    // ------- infrastructure level errors
    #[error("window not found")]
    WindowNotFound,
}
