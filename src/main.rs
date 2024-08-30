use std::io::Read;
use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::Digest;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use solana_sdk::commitment_config::CommitmentConfig;

mod note;
mod create_metadata;
mod ido;
mod common;

// const RPC_ADDR: &str = "https://api.devnet.solana.com";
const RPC_ADDR: &str = "http://localhost:8899";

fn main() {
    // let x = [216, 9, 153, 85, 119, 98, 16, 211, 95, 29, 77, 219, 43, 104, 4, 73, 230, 209, 91, 135, 60, 154, 109, 32, 41, 21, 174, 51, 96, 111, 158, 68, 250, 44, 199, 216, 162, 104, 240, 42, 34, 140, 39, 197, 56, 75, 252, 127, 246, 202, 60, 15, 17, 188, 19, 195, 120, 196, 118, 254, 240, 110, 81, 157];
    // println!("pda: {}", Keypair::from_bytes(&x).unwrap().pubkey().to_string());
    let client = RpcClient::new_with_commitment(RPC_ADDR.to_string(), CommitmentConfig::confirmed());
    let payer = Keypair::from_base58_string("63gS1D49STGGYkKH7SoVgtf628HjbqjqPnwYPAXLFYEjPqY6ENyR1SGUGRL3kXmLUp9Lw6Jr3oKo9vb3zXv2VNXZ");

    // note::test(&client, &payer);
    // create_metadata::test(&client, &payer);
    ido::test(&client, &payer);
}



