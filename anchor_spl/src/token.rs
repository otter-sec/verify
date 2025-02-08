use onchor::solana_program::account_info::AccountInfo;

use onchor::solana_program::program_pack::Pack;
use onchor::solana_program::pubkey::Pubkey;
use onchor::system_program::Transfer;
use onchor::{context::CpiContext, Accounts};
use onchor::{solana_program, Result};
use std::ops::Deref;

use crate::token_2022::TransferChecked;

solana_program::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");


pub fn transfer<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, Transfer<'info>>,
    _amount: u64,
) -> Result<()> {
    Ok(())
}

pub fn transfer_checked<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, TransferChecked<'info>>,
    _amount: u64,
    _decimals: u8,
) -> Result<()> {
    Ok(())
}


pub mod accessor {
    use super::*;

    // u8 = 1 byte starts from 3rd byte of data
    pub fn amount(account: &AccountInfo) -> Result<u64> {
        let bytes = account.try_borrow_data()?;
        let mut amount_bytes = [0u8; 8];
        amount_bytes.copy_from_slice(&bytes[2..10]);
        Ok(u64::from_le_bytes(amount_bytes))
    }

    // 1st byte of data is the mint key
    pub fn mint(account: &AccountInfo) -> Result<Pubkey> {
        let bytes = account.try_borrow_data()?;
        Ok(Pubkey::new_from_array([bytes[0]]))
    }

    // 2nd byte of data is the authority key
    pub fn authority(account: &AccountInfo) -> Result<Pubkey> {
        let bytes = account.try_borrow_data()?;
        Ok(Pubkey::new_from_array([bytes[1]]))
    }
}
