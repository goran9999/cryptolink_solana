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
    #[error("MessageV3: Invalid account seeds")]
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
    #[error("MessageV3: Invalid instruction")]
    InvalidInstruction,
    #[error("MessageV3: Missing validation account info!")]
    MissingValidationAccountInfo,
    #[error("MessageV3: Invalid client program id!")]
    InvalidClientProgramId,
    #[error("MessageV3: Invalid client program update authority!")]
    InvalidUpdateAuthority,
    #[error("MessageV3: Message already processed!")]
    MessageAlreadyProcessed,
}

impl From<MessengerError> for ProgramError {
    fn from(value: MessengerError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
