use {
    crate::{
        error::SolTradeError,
        state::{Authority, Trade},
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(index: u32)]
pub struct CreateTradeContext<'info> {
    // ? authority: account to create
    #[account(
        mut,
        seeds = [
            Authority::PREFIX.as_bytes(),
            crate::id().as_ref(),
        ],
        bump,
    )]
    pub authority: Account<'info, Authority>,

    // ? trade: account to create
    #[account(
        init,
        payer = user,
        seeds = [
            Trade::PREFIX.as_bytes(),
            index.to_le_bytes().as_ref(),
        ],
        space = Trade::SIZE,
        bump,
    )]
    pub trade: Account<'info, Trade>,

    // ? user: who is creating the trade
    #[account(mut)]
    pub user: Signer<'info>,

    // ? system_program: system program
    pub system_program: Program<'info, System>,
}

pub fn create_trade(
    ctx: Context<CreateTradeContext>,
    trade_index: u32,
    allowed_users: Vec<Pubkey>,
) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    let trade = &mut ctx.accounts.trade;
    let user = &mut ctx.accounts.user;

    // ! @check
    // |-allowed users max 5
    require!(
        allowed_users.len() <= 5,
        SolTradeError::CreateTradeTooManyAllowedUsers
    );
    // |-allowed users not contains user
    require!(
        !allowed_users.contains(&user.key()),
        SolTradeError::CreateTradeNotAllowedUser
    );

    // * create trade account
    trade.index = trade_index;
    trade.user = user.key();
    trade.offer_count = 0;
    trade.asset_count = 0;
    trade.enabled = false;
    trade.allowed_users = allowed_users;
    trade.asset_sol_count = 0;
    trade.asset_spl_count = 0;
    trade.exchanged_count = 0;

    authority.trade_count = authority.trade_count.checked_add(1).unwrap();

    Ok(())
}
