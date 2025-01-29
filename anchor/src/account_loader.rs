use std::marker::PhantomData;

use otter_solana_program::{account_info::AccountInfo, error::Error, Result};

use crate::Owner;

#[derive(Clone)]
pub struct AccountLoader<'info, T: Owner> {
    acc_info: &'info AccountInfo<'info>,
    phantom: PhantomData<&'info T>,
}

impl<'info, T: Owner> AccountLoader<'info, T> {
    pub fn new(acc_info: &'info AccountInfo<'info>) -> Self {
        Self { acc_info, phantom: PhantomData }
    }

     #[inline(never)]
    pub fn try_from(acc_info: &'info AccountInfo<'info>) -> Result<AccountLoader<'info, T>> {
        if acc_info.owner != &T::owner() {
            return Err(Error::Generic);
        }
        Ok(Self::new(acc_info))
    }

    #[inline(never)]
    pub fn try_from_unchecked(acc_info: &'info AccountInfo<'info>) -> AccountLoader<'info, T> {
        Self::new(acc_info)
    }
}