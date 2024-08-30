use std::str::FromStr;
use std::time::Duration;

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use crate::common::calculate_discriminator;

pub fn test(rpc_client: &RpcClient, payer: &Keypair) {
    let program_id = solana_program::pubkey::Pubkey::from_str("CKyE2drXuaYcbBF8japHFSQTgEShYfGBtZKjJuN1nMT3").unwrap();
    let (pda, _bump) = Pubkey::find_program_address(&[
        &payer.pubkey().as_ref(),
    ], &program_id);
    println!("pda: {}", pda.to_string());
    create(&rpc_client, &program_id, &payer, &pda);
    std::thread::sleep(Duration::from_secs(5));
    reading(&rpc_client, &pda);
    modification(&rpc_client, &program_id, &payer, &pda);
    std::thread::sleep(Duration::from_secs(5));
    reading(&rpc_client, &pda);
    std::thread::sleep(Duration::from_secs(5));
    delete(&rpc_client, &program_id, &payer, &pda);
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Notes {
    pub notes: String,
}

fn reading(client: &RpcClient, pda: &solana_program::pubkey::Pubkey) {
    let discriminator_len = 8;
    // 读取并解析账户数据
    let account_data = client.get_account_data(&pda).unwrap();
    let length = u32::from_le_bytes(account_data[0 + discriminator_len..4 + discriminator_len].try_into().unwrap()) as usize;
    let v = &account_data[0 + discriminator_len..length + 4 + discriminator_len];
    let greeting_account: Notes = Notes::try_from_slice(v).unwrap();
    println!("读取数据: {} ", greeting_account.notes);
}

fn delete(client: &RpcClient, program_id: &solana_program::pubkey::Pubkey, payer: &Keypair, pda: &solana_program::pubkey::Pubkey) {
    // 构建交易
    let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        *program_id,
        &calculate_discriminator("delete"),
        vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(*pda, false),
        ],
    );
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}

fn modification(client: &RpcClient, program_id: &solana_program::pubkey::Pubkey, payer: &Keypair, pda: &solana_program::pubkey::Pubkey) {
    // 构建交易
    let mut vec = calculate_discriminator("modification").to_vec();
    let mut data = borsh::to_vec(&Notes { notes: "修改后的修改后的".to_string() }).unwrap();
    println!("modification data: {:?} ", &vec);
    vec.append(&mut data);
    let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        *program_id,
        vec.as_slice(),
        vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(*pda, false),
            solana_sdk::instruction::AccountMeta::new(system_program::ID, false),
        ],
    );
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
    // 读取并解析账户数据
    let account_data = client.get_account_data(&pda).unwrap();
    println!("account_data {:?} ", &account_data[..]);
}

fn create(client: &RpcClient, program_id: &solana_program::pubkey::Pubkey, payer: &Keypair, pda: &solana_program::pubkey::Pubkey) {
    // 构建交易
    let mut vec = calculate_discriminator("create").to_vec();
    let mut data = borsh::to_vec(&Notes { notes: "创建的".to_string() }).unwrap();
    vec.append(&mut data);
    println!("create data: {:?} ", &vec);
    let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        *program_id,
        vec.as_slice(),
        vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(*pda, false),
            solana_sdk::instruction::AccountMeta::new(system_program::ID, false),
        ],
    );
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
    // 读取并解析账户数据
    let account_data = client.get_account_data(&pda).unwrap();
    println!("account_data {:?} ", &account_data[..]);
}

