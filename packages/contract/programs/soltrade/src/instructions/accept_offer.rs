use {
    crate::state::{Offer, Trade},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct AcceptOfferContext<'info> {
    // ? offer: account to create
    #[account(mut)]
    pub offer: Account<'info, Offer>,

    // ? trade: trade account
    // #[account(mut, seeds = [Trade::PREFIX.as_bytes()], bump)]
    #[account(mut)]
    pub trade: Account<'info, Trade>,

    // ? user: who is creating the offer
    #[account(mut)]
    pub user: Signer<'info>,

    // ? system_program: system program
    pub system_program: Program<'info, System>,
}

pub fn accept_offer(ctx: Context<AcceptOfferContext>) -> Result<()> {
    // let user = &mut ctx.accounts.user;
    let offer = &mut ctx.accounts.offer;
    let trade = &mut ctx.accounts.trade;

    // ! @check
    // |-index should be same as current asset count in trade
    // require!(
    //     offer.asset_count == index,
    //     SolTradeError::AcceptOfferInvalidIndex
    // );

    trade.accepted_offer = offer.key();
    offer.accepted = true;

    Ok(())
}
