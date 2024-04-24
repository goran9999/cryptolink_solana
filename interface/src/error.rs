//! Error types

use spl_program_error::*;

#[spl_program_error()]
pub enum MessageHookError {
    /// Incorrect account provided
    #[error("Incorrect account provided")]
    IncorrectAccount,
}
