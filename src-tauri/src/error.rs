use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MrDamianError {
    // TODO: consider about error category and boundary.

    // ------- domain level errors
    #[error("message key not found")]
    MessageKeyNotFound,

    // ------- infrastructure level errors
    #[error("window not found")]
    WindowNotFound,
}
