use std::io::Read;
use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::Digest;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use solana_sdk::commitment_config::CommitmentConfig;

mod note;
mod create_metadata;

// const RPC_ADDR: &str = "https://api.devnet.solana.com";
const RPC_ADDR: &str = "http://localhost:8899";

fn main() {
    let client = RpcClient::new_with_commitment(RPC_ADDR.to_string(), CommitmentConfig::confirmed());
    let payer = Keypair::from_base58_string("63gS1D49STGGYkKH7SoVgtf628HjbqjqPnwYPAXLFYEjPqY6ENyR1SGUGRL3kXmLUp9Lw6Jr3oKo9vb3zXv2VNXZ");

    note::test(&client, &payer);
    create_metadata::test(&client, &payer);
}



