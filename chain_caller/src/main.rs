//! Example program to show that anchor program is setup correctly.

use anyhow::Result;
use borsh::BorshSerialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{fs, str::FromStr};

const PROGRAM_ID: &str = "5bKcGNx6FVX6raPWZCXWjjLEPZpTfMVRM7xomhkWcLF";
/// Should be filled as absolute path
///
/// ```rust
/// const KEYPAIR_PATH: &str = "/home/${USER}/.config/solana/id.json";
/// ```
const KEYPAIR_PATH: &str = "~/.config/solana/id.json";

/// From IDL
const ADD_ASSET_PAIR_DISCRIMINATOR: [u8; 8] = [22, 78, 18, 236, 70, 205, 173, 25];
/// Seed = "asset_holder"
const ASSET_HOLDER_SEED: &[u8] = b"asset_holder";

#[derive(BorshSerialize)]
pub struct AddAssetPairIx {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}

fn main() -> Result<()> {
    let rpc_url = "http://127.0.0.1:8899";
    let client = RpcClient::new(rpc_url.to_string());

    let payer = {
        let bytes = fs::read(KEYPAIR_PATH)?;
        Keypair::from_bytes(&serde_json::from_slice::<Vec<u8>>(&bytes)?)?
    };
    client.request_airdrop(&payer.pubkey(), 2_000_000_000)?;

    let program_id = Pubkey::from_str(PROGRAM_ID)?;

    let (asset_holder_pda, _bump) = Pubkey::find_program_address(&[ASSET_HOLDER_SEED], &program_id);

    println!("asset_holder PDA: {}", asset_holder_pda);

    // Example real-looking mint pubkeys (can be anything on localnet)
    let base_mint = Pubkey::from_str("So11111111111111111111111111111111111111112")?;
    let quote_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2q9p5Ggq5sF7p7x4F9K7KkJc9w")?;

    let ix_args = AddAssetPairIx {
        base_mint,
        quote_mint,
        base_decimals: 9,
        quote_decimals: 6,
    };

    let mut data = Vec::new();
    data.extend_from_slice(&ADD_ASSET_PAIR_DISCRIMINATOR);
    data.extend(ix_args.try_to_vec()?);

    let add_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(asset_holder_pda, false),
            AccountMeta::new(payer.pubkey(), true),
        ],
        data,
    };

    let recent_blockhash = client.get_latest_blockhash()?;

    let add_tx = Transaction::new_signed_with_payer(
        &[add_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let add_sig = client.send_and_confirm_transaction(&add_tx)?;
    println!("add_asset_pair sig: {}", add_sig);

    Ok(())
}
