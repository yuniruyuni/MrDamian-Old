use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MrDamianError {
    #[error("window not found")]
    WindowNotFound,
}
