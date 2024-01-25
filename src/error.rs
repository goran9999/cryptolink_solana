use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum MessengerError {
    #[error("MessageV3: Pubkey missmatch")]
    PublicKeyMissmatch,
    #[error("MessageV3: Bridge is not enabled")]
    BrigdeNotEnabled,
    #[error("MessageV3: caller is not an operator")]
    CallerNotOperator,
    #[error("MessageV3: caller is not an ateam")]
    CallerNotATeam,
    #[error("MessageV3: caller is not a super")]
    CallerNotSuper,
    #[error("MessageV3: chain not supported")]
    ChainNotSupported,
    #[error("MessageV3: caller not whitelisted")]
    CallerNotWhitelisted,
    #[error("MessageV3: Config already initialized")]
    ConfigInitialized,
    #[error("MessageV3: Invalid ccount seeds")]
    InvalidAccountSeeds,
    #[error("MessageV3: Account not signer")]
    AccountNotSigner,
    #[error("MessageV3: Invalid role for action")]
    InvalidRoleForAction,
    #[error("MessageV3: Invalid instruction index")]
    InvalidInstructionIndex,
    #[error("MessageV3: Invalid pre-instruction")]
    InvalidPreInstruction,
    #[error("MessageV3: Exsig already added for given recipient")]
    ExsigExists,
    #[error("MessageV3: Invalid signature")]
    InvalidSignature,
}

impl From<MessengerError> for ProgramError {
    fn from(value: MessengerError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
