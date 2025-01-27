use onchor::{
    prelude::{AccountInfo, AccountMeta, CpiContext},
    ToAccountInfos, ToAccountMetas,
};

use crate::Result;

#[derive(Debug)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Transfer<'info> {
    pub from: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}

impl ToAccountMetas for Transfer<'_> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.from.is_signer);
        let from = self.from.to_account_meta(is_signer);
        let to = self.to.to_account_meta(false);
        let authority = self.authority.to_account_meta(false);
        vec![from, to, authority]
    }
}


impl<'info> ToAccountInfos<'info> for Transfer<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![
            self.from.clone(),
            self.to.clone(),
            self.authority.clone(),
        ]
    }
}


#[derive(Debug)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct TransferChecked<'info> {
    pub from: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}

impl ToAccountMetas for TransferChecked<'_> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.from.is_signer);
        let from = self.from.to_account_meta(is_signer);
        let to = self.to.to_account_meta(false);
        let mint = self.mint.to_account_meta(false);
        let authority = self.authority.to_account_meta(false);
        vec![from, to, mint, authority]
    }
}

impl<'info> ToAccountInfos<'info> for TransferChecked<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![
            self.from.clone(),
            self.to.clone(),
            self.mint.clone(),
            self.authority.clone(),
        ]
    }
}


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
