// use std::str::FromStr;

// use mv3_contract_solana::instruction::{change_config, ChangeConfig};
// use serde_json::from_str;
// use solana_client::rpc_client::RpcClient;
// use solana_program::pubkey::Pubkey;
// use solana_program_test::tokio;
// use solana_sdk::{
//     signer::{keypair::Keypair, Signer},
//     transaction::Transaction,
// };
// use std::fs;

// #[tokio::main]

// pub async fn main() {
//     let program_id = Pubkey::from_str("mv3PxTJXnsExfkFtwbKCo35fGKdtfcowo9xZsmXQ2qJ").unwrap();

//     let raw_keypair = serde_json::from_str::<Vec<u8>>(
//         &fs::read_to_string("tests/wallets/authority.json")
//             .unwrap()
//             .to_string(),
//     )
//     .unwrap();

//     let raw_exsig = from_str::<Vec<u8>>(
//         fs::read_to_string("tests/wallets/exsig.json")
//             .unwrap()
//             .as_str(),
//     )
//     .unwrap();

//     let keypair = Keypair::from_bytes(&raw_keypair).unwrap();

//     let rpc_connection = RpcClient::new(String::from(
//         "https://devnet.helius-rpc.com/?api-key=3be4032a-b6d2-475e-8023-406c93f7937b",
//     ));

//     let exsig: [u8; 32] = raw_exsig.as_slice().try_into().unwrap();

//     let ix = change_config(
//         program_id,
//         keypair.pubkey(),
//         ChangeConfig {
//             enabled_chains: None,
//             bridge_enabled: None,
//             accountant: None,
//             whitelist_only: None,
//             chainsig: Some(exsig),
//         },
//     );

//     let mut transaction = Transaction::new_with_payer(&[ix], Some(&keypair.pubkey()));

//     let recent_blockhash = rpc_connection.get_latest_blockhash().unwrap();

//     transaction.sign(&[&keypair], recent_blockhash);

//     let tx_sig = rpc_connection
//         .send_and_confirm_transaction(&transaction)
//         .unwrap();

//     println!("TX SIG: {:?}", tx_sig.to_string());
// }
