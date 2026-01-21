#![allow(unexpected_cfgs)]

use std::collections::BTreeMap;

use anchor_lang::prelude::*;

declare_id!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

pub const PREFIX_SIZE: usize = 4;
pub const MAX_ASSET_PAIRS: usize = 30;
pub const DISCRIMINATOR_SIZE: usize = 8;

#[account]
pub struct AssetHolder {
    pub bump: u8,
    pub authority: Pubkey,
    pub pairs: BTreeMap<(Pubkey, Pubkey), (u8, u8)>,
}

#[derive(Accounts)]
pub struct AddAssetPair<'info> {
    #[account(
        mut,
        constraint = asset_holder.authority == authority.key()
    )]
    pub asset_holder: Account<'info, AssetHolder>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"seed"],
        bump,
        payer = authority,
        space = AssetHolder::LEN
    )]
    pub asset_holder: Account<'info, AssetHolder>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[program]
pub mod asset_holder {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let asset_holder = &mut ctx.accounts.asset_holder;

        asset_holder.bump = ctx.bumps.asset_holder;
        asset_holder.authority = ctx.accounts.authority.key();
        asset_holder.pairs = BTreeMap::new();

        Ok(())
    }

    pub fn add_asset_pair(
        _ctx: Context<AddAssetPair>,
        _base_mint: Pubkey,
        _quote_mint: Pubkey,
        _base_decimals: u8,
        _quote_decimals: u8,
    ) -> Result<()> {
        Ok(())
    }
}

// Storage len
pub trait StorageType {
    const LEN: usize;
}
impl StorageType for u8 {
    const LEN: usize = 1;
}

impl StorageType for Pubkey {
    const LEN: usize = 32;
}

impl<A, B> StorageType for (A, B)
where
    A: StorageType,
    B: StorageType,
{
    const LEN: usize = A::LEN + B::LEN;
}

impl<K, V> StorageType for BTreeMap<K, V>
where
    K: StorageType,
    V: StorageType,
{
    const LEN: usize = PREFIX_SIZE + (MAX_ASSET_PAIRS * (K::LEN + V::LEN));
}

impl StorageType for AssetHolder {
    const LEN: usize =
        DISCRIMINATOR_SIZE + u8::LEN + Pubkey::LEN + BTreeMap::<(Pubkey, Pubkey), (u8, u8)>::LEN;
}
