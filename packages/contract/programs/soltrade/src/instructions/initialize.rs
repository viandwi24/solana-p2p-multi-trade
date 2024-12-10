use {
    crate::{error::SolTradeError, state::Authority},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    // ? authority: account to create
    #[account(
        init,
        payer = user,
        seeds = [
            Authority::PREFIX.as_bytes(),
            crate::id().as_ref(),
        ],
        space = Authority::SIZE,
        bump,
    )]
    pub authority: Account<'info, Authority>,

    // ? user: who is creating the trade
    #[account(mut)]
    pub user: Signer<'info>,

    // ? system_program: system program
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
    let authority = &mut ctx.accounts.authority;

    authority.user = *ctx.accounts.user.key;
    authority.trade_count = 0;

    Ok(())
}
