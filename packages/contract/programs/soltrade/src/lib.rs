use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("AHwXuw8go83wWKF8PSXeMAzfaPhxF5t3GH7n4jQRPpJW");

#[program]
pub mod soltrade {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        return instructions::initialize(ctx);
    }

    pub fn create_trade(
        ctx: Context<CreateTradeContext>,
        trade_index: u32,
        allowed_users: Vec<Pubkey>,
    ) -> Result<()> {
        return instructions::create_trade(ctx, trade_index, allowed_users);
    }

    pub fn create_offer(ctx: Context<CreateOfferContext>, index: u32) -> Result<()> {
        return instructions::create_offer(ctx, index);
    }

    pub fn add_asset_item(
        ctx: Context<AddAssetItemContext>,
        from_type: u8,
        index: u32,
        asset_type_index: u32,
        asset_type: u8,
        options: Vec<u8>,
    ) -> Result<()> {
        return instructions::add_asset_item(
            ctx,
            from_type,
            index,
            asset_type_index,
            asset_type,
            options,
        );
    }

    pub fn accept_offer(ctx: Context<AcceptOfferContext>) -> Result<()> {
        return instructions::accept_offer(ctx);
    }

    pub fn exchange(
        ctx: Context<ExchangeContext>,
        trade_index: u32,
        offer_index: u32,
        from_type: u8,
        asset_type: u8,
        asset_index: u32,
        asset_type_index: u32,
    ) -> Result<()> {
        return instructions::exchange(
            ctx,
            trade_index,
            offer_index,
            from_type,
            asset_type,
            asset_index,
            asset_type_index,
        );
    }
}

#[derive(Accounts)]
pub struct Initialize {
    // pub trade: UncheckedAccount<'info>,
}
