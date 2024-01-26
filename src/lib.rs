pub mod constants;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

pub use solana_program;
use solana_program::declare_id;

declare_id!("mv3PxTJXnsExfkFtwbKCo35fGKdtfcowo9xZsmXQ2qJ");
