# Hello NFT contract

**Hello NFT** is smart contract build on Solana that is used as part of Cryptolink program library for bridging Non-Fungible tokens
from EVM chains to Solana and opposite.

## Processors

- init_collection

```rust
pub fn process_init_collection(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: InitCollection,
) -> ProgramResult;
```

> Can be executed once per program deployment, is used for setting up **collection mint** on Solana, such as defining name and symbol
> of collection that will be bridged from EVM chain. Signer of transaction is authority of mint configuration,while update authority of
> newly created collection is assigned to PDA (program derived address).

- initialize_extra_account_meta

```rust

pub fn process_initialize_extra_account_meta_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    extra_account_metas: &[ExtraAccountMeta],
) -> ProgramResult;

```

> Instruction that is part of MessageHook[] interface, built by Cryptolink for easier interaction with Solana smart contracts in cross chain
> apps and for more standardized set of instructions. This instruction is responsible for creating program derived address that is storing
> all accounts required by process_mint_nft. It is utilizing Solana TLV Account resolution for creating this PDA, just as newly
> published token program,also known as Token2022, built by Solana core team does.

- mint_nft

```rust
pub fn process_mint_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> ProgramResult
```

> Presents core instruction of this smart contract that is responsible for minting NFT on Solana, by receiving message emitted from EVM chain
> that is forwarded using Cryptolink infrastructure! Most important part of instruction is **data** argument,which holds encoded **TokenURI**
> of Non-Fungible Token, such as bytes32 encoded Solana address of recipient. Due to TLV account resolution infrastructure, NFT is not directly
> minted to a wallet but new _proof_ PDA is created that holds NFT and only authorized wallet can withdraw one from it!

## Instructions

For easier integration and starting process of bridging NFTs from EVM to Solana, refer to **src/instructions.rs** file.
Those functions are responsible for creating instruction with all required accounts so they can be used and inserted into
transaction which can be easily executed.

1. init_collection

```rust
pub fn init_collection(
    program_id: Pubkey,
    authority: &Pubkey,
    collection: &Pubkey,
    metadata: &Pubkey,
    edition: &Pubkey,
    token_record: &Pubkey,
    authorization_rules: &Pubkey,
    token_account: &Pubkey,
    collection_config: InitCollection,
) -> Instruction
```

- program_id - Address of your deployed contract,found in entrypoint.rs file,after you deploy your contract.
- authority - Public address of authority wallet that will be responsible for setting configuration of bridge.
- collection - Address of collection nft which can be either randomly generated pubkey or grinded on.
- metadata - Metadata of collection nft,account owned by [Metaplex Program](https://github.com/metaplex-foundation/mpl-token-metadata).
  Rules for deriving PDA address of this account can be found here: [Metadata PDA](https://developers.metaplex.com/token-metadata/)
- edition - Another account owned by, with different set of seeds
- token_record - Account owned by Metaplex that is storing data about NFT ownership such as delegation rules and token locking.
- authorization_rules - Rules account used to enforce royalties on NFTs on secondary marketplaces. Two most popular authorization
  rules accounts are: - Metaplex Foundation Rule Set (eBJLFYPxJmMGKuFwpDWkzxZeUrad92kZRC5BJLpzyT9) and Compatability Rule Set (AdH2Utn6Fus15ZhtenW4hZBQnvtLgM1YCW2MfVp7pYS5)
- token_account - Account that will hold actual token of collection NFT that will be created inside given instruction. It needs to be [Associated Token Account](https://spl.solana.com/associated-token-account) for easier managing and transferring of this asset!
- collection_config - Account owned by **HelloNFT** program that holds actual configuration of newly generated collection on Solana blockchain.

```rust
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CollectionConfig {
    pub collection_key: Pubkey,
    pub total_minted: u32,
    pub authority: Pubkey,
    pub collection_name: String,
    pub collection_symbol: String,
    pub total_supply: u32,
};
```

## Usage

For using this smart contract and deploying it to Solana blockchain, several dependencies need to be installed on host system.

1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

```bash
rustup install 1.75.0
rustup default 1.75.0
```

2. Install Solana CLI

```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"
```

3. Generate program ID

```bash
solana-keygen new -o ./program.json
```

This will give output of program pubkey that needs to be changed in **declare_id!()**
macro in src/lib.rs

4. Build contract

```bash
cargo build-sbf
```

5. Deploy contract

For this command,make sure to switch CLI to devnet when in testing phase,by running:

```bash

solana config set --url https://api.devnet.solana.com
```

Also, make sure to have enough devnet SOL on your CLI wallet. You can get up to 2 SOL/h
by running

```bash
solana airdrop 2
```

For getting your CLI wallet address and sending externally SOL to it, you can run

```bash
solana address
```

6. Deploying contract

```bash
solana program deploy --program-id ./program.json ./target/deploy/hello_nft.so
```
