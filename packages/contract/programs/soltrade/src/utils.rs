use anchor_lang::prelude::*;

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
    if account.owner != owner {
        return Err(ProgramError::IllegalOwner.into());
    } else {
        return Ok(());
    }
}
