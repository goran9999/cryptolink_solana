use std::{fmt::Error, str::FromStr};

use mv3_contract_solana::processor::process_instruction;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, transaction::Transaction};
mod process_create_config;
mod utils;
