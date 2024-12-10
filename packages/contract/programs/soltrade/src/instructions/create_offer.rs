use {
    crate::{
        error::SolTradeError,
        state::{Offer, Trade},
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(index: u32)]
pub struct CreateOfferContext<'info> {
    // ? offer: account to create
    #[account(
        init,
        payer = user,
        seeds = [
            trade.key().as_ref(),
            Offer::PREFIX.as_bytes(),
            index.to_le_bytes().as_ref(),
        ],
        space = Offer::SIZE,
        bump,
    )]
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

pub fn create_offer(ctx: Context<CreateOfferContext>, index: u32) -> Result<()> {
    let offer = &mut ctx.accounts.offer;
    let user = &mut ctx.accounts.user;
    let trade = &mut ctx.accounts.trade;

    // ! @check
    // |-index should be same as current asset count in trade
    require!(
        offer.asset_count == index,
        SolTradeError::CreateOfferInvalidIndex
    );

    // * create trade account
    offer.index = index;
    offer.user = user.key();
    offer.asset_count = 0;
    offer.enabled = false;
    offer.asset_sol_count = 0;
    offer.asset_spl_count = 0;
    offer.exchanged_count = 0;
    offer.accepted = false;

    // * increase offer count
    trade.offer_count = trade.offer_count.checked_add(1).unwrap();

    Ok(())
}
