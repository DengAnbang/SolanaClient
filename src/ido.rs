use std::fmt::Debug;
use std::str::FromStr;
use std::time::Duration;

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::Digest;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::calculate_discriminator;

pub fn test(rpc_client: &RpcClient, payer: &Keypair) {
    let program_id = solana_program::pubkey::Pubkey::from_str("HqaXfTZYwEaAu9fmjjauUZ6zsaGJPB65SgFUbahETvcU").unwrap();
    let (pda, _bump) = Pubkey::find_program_address(&[
        b"ido".as_slice(),
    ], &program_id);
    // println!("pda: {}", pda.to_string());

    // reading::<IdoAccount>(&rpc_client, &pda);

    delete(&rpc_client, &program_id, &payer, &pda);
    std::thread::sleep(Duration::from_secs(5));
    initialize(rpc_client, &program_id, payer, &pda);
    std::thread::sleep(Duration::from_secs(5));
    reading::<IdoAccount>(&rpc_client, &pda);
    std::thread::sleep(Duration::from_secs(5));
    add_whitelist(&rpc_client, &program_id, &payer, &pda, vec![program_id,
                                                               solana_program::pubkey::Pubkey::from_str("65a63S4TJFL61Vsi4qmiJmzZ3LsievWSUeAJoC79xzSj").unwrap(),
    ]);
    std::thread::sleep(Duration::from_secs(5));
    reading::<IdoAccount>(&rpc_client, &pda);
    // std::thread::sleep(Duration::from_secs(5));
    // delete(&rpc_client, &program_id, &payer, &pda);
}

fn initialize(rpc_client: &RpcClient, program_id: &Pubkey, payer: &Keypair, pda: &Pubkey) {
    // 构建交易
    let mut vec = calculate_discriminator("initialize").to_vec();
    let ido_account = IdoAccount {
        owner: payer.pubkey(),
        sales_token: Pubkey::from_str("8wN7QfNHcV8DzRZah2KgGuKPhftViryjpvjKJWS5nSrD").unwrap(),
        tokens_to_sell: 100,
        ethers_to_raise: 10,
        refund_threshold: 5,
        min_commit: 1,
        max_commit: 10,
        emission_token: Pubkey::from_str("8wN7QfNHcV8DzRZah2KgGuKPhftViryjpvjKJWS5nSrD").unwrap(),
        emission_total: 0,
        burn_address: Pubkey::from_str("8wN7QfNHcV8DzRZah2KgGuKPhftViryjpvjKJWS5nSrD").unwrap(),
        start_time: 0,
        end_time: 0,
        receive_time: 0,
        whitelist: Vec::new(),
    };


    let mut data = borsh::to_vec(&ido_account).unwrap();
    println!("create data: {:?} ", &data);
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
    let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    // 发送交易
    let signature = rpc_client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}

fn add_whitelist(rpc_client: &RpcClient, program_id: &Pubkey, payer: &Keypair, pda: &Pubkey, whitelist: Vec<Pubkey>) {
    // 构建交易
    let mut vec = calculate_discriminator("add_whitelist").to_vec();


    let mut data = borsh::to_vec(&whitelist).unwrap();
    println!("create data: {:?} ", &data);
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
    let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    // 发送交易
    let signature = rpc_client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}


fn reading<T: BorshDeserialize + Debug>(client: &RpcClient, pda: &Pubkey) {
    // 读取并解析账户数据
    let account_data = client.get_account_data(&pda).unwrap();
    println!("读取数据: {:?} ", account_data);
    let result = IdoAccount::try_from_slice(&account_data[8..]);
    println!("解析数据: {:?} ", result.unwrap());
    // let payer = Keypair::from_base58_string("63gS1D49STGGYkKH7SoVgtf628HjbqjqPnwYPAXLFYEjPqY6ENyR1SGUGRL3kXmLUp9Lw6Jr3oKo9vb3zXv2VNXZ");
    // let ido_account = IdoAccount {
    //     owner: payer.pubkey(),
    //     sales_token: Pubkey::from_str("8wN7QfNHcV8DzRZah2KgGuKPhftViryjpvjKJWS5nSrD").unwrap(),
    //     tokens_to_sell: 100,
    //     ethers_to_raise: 10,
    //     refund_threshold: 5,
    //     min_commit: 1,
    //     max_commit: 10,
    //     emission_token: Pubkey::from_str("8wN7QfNHcV8DzRZah2KgGuKPhftViryjpvjKJWS5nSrD").unwrap(),
    //     emission_total: 0,
    //     burn_address: Pubkey::from_str("8wN7QfNHcV8DzRZah2KgGuKPhftViryjpvjKJWS5nSrD").unwrap(),
    //     start_time: 0,
    //     end_time: 0,
    //     receive_time: 0,
    //     whitelist: Vec::new(),
    // };
    // let mut data = borsh::to_vec(&ido_account).unwrap();
    // println!("create data: {:?} ", &data);
    // println!("create data: {:?} ", &IdoAccount::try_from_slice(&data));
    // let x1 = [149, 120, 27, 240, 23, 207, 180, 98, 75, 118, 43, 92, 106, 223, 29, 64, 94, 159, 193, 125, 116, 6, 147, 101, 206, 169, 6, 254, 180, 111, 158, 24, 146, 217, 139, 115, 232, 110, 250, 112, 117, 239, 10, 184, 42, 47, 88, 22, 223, 11, 31, 222, 85, 6, 217, 149, 91, 218, 206, 234, 156, 157, 202, 110, 124, 23, 10, 22, 80, 226, 231, 82, 100, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 117, 239, 10, 184, 42, 47, 88, 22, 223, 11, 31, 222, 85, 6, 217, 149, 91, 218, 206, 234, 156, 157, 202, 110, 124, 23, 10, 22, 80, 226, 231, 82, 0, 0, 0, 0, 0, 0, 0, 0, 117, 239, 10, 184, 42, 47, 88, 22, 223, 11, 31, 222, 85, 6, 217, 149, 91, 218, 206, 234, 156, 157, 202, 110, 124, 23, 10, 22, 80, 226, 231, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 250, 44, 199, 216, 162, 104, 240, 42, 34, 140, 39, 197, 56, 75, 252, 127, 246, 202, 60, 15, 17, 188, 19, 195, 120, 196, 118, 254, 240, 110, 81, 157, 75, 118, 43, 92, 106, 223, 29, 64, 94, 159, 193, 125, 116, 6, 147, 101, 206, 169, 6, 254, 180, 111, 158, 24, 146, 217, 139, 115, 232, 110, 250, 112];
    //
    // let result = IdoAccount::try_from_slice(&x1[8..]);
    // println!("解析数据: {:?} ", result.unwrap());
}


fn delete(client: &RpcClient, program_id: &Pubkey, payer: &Keypair, pda: &Pubkey) {
    // 构建交易
    let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        *program_id,
        &calculate_discriminator("delete"),
        vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(*pda, false),
            solana_sdk::instruction::AccountMeta::default(),
        ],
    );
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct IdoAccount {
    pub owner: Pubkey, //所有者的地址
    pub sales_token: Pubkey, //售卖的token地址
    pub tokens_to_sell: u64, //售卖的token数量  需要加18个0
    pub ethers_to_raise: u64, //需要募集的sol，即硬顶
    pub refund_threshold: u64, //退款阈值，即软顶 ，私募时为0 ，若募集少于此金额，用户资金全款退还，募集失败
    pub min_commit: u64, // 单次购买最小金额
    pub max_commit: u64, // 最多购买金额
    pub emission_token: Pubkey, //激励token地址  本次活动与 _salesToken保持一致
    pub emission_total: u64, // 激励token数量
    pub burn_address: Pubkey, //黑洞地址 固定写 0x000000000000000000000000000000000000dEaD
    pub start_time: u64, //活动开始时间unix时间戳
    pub end_time: u64, //活动结束时间
    pub receive_time: u64, // token领取时间
    pub whitelist: Vec<Pubkey>, //白名单
}