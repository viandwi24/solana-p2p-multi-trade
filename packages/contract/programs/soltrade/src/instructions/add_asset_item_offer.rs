use {
    crate::{
        error::SolTradeError,
        state::{AssetItemSOL, AssetItemSPL, AssetItemType, Offer},
        utils::assert_owned_by,
    },
    anchor_lang::prelude::*,
    borsh::{BorshDeserialize, BorshSerialize},
    std::str::FromStr,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OptionsAssetItemSOL {
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OptionsAssetItemSPL {
    pub token: String,
    pub mint: String,
}

#[derive(Accounts)]
#[instruction(_index: u32, asset_type_index: u32, asset_type: u8)]
pub struct AddAssetItemOfferContext<'info> {
    // ? trade: trade or offer account to add asset item
    #[account(mut, seeds = [Offer::PREFIX.as_bytes()], bump)]
    pub offer: Account<'info, Offer>,

    // ? just for struct
    // pub asset_item_base: Option<Account<'info, BaseAssetItem>>,
    #[account(
        init,
        payer = user,
        space = AssetItemSOL::SIZE,
        seeds = [
            offer.key().as_ref(),
            AssetItemSOL::PREFIX.as_bytes(),
            asset_type_index.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub asset_item_sol: Option<Account<'info, AssetItemSOL>>,

    #[account(
        init,
        payer = user,
        space = AssetItemSPL::SIZE,
        seeds = [
            offer.key().as_ref(),
            AssetItemSPL::PREFIX.as_bytes(),
            asset_type_index.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub asset_item_spl: Option<Account<'info, AssetItemSPL>>,

    // ? user: who is creating the trade
    #[account(mut)]
    pub user: Signer<'info>,

    // ? system_program: system program
    pub system_program: Program<'info, System>,
}

pub fn add_asset_item_offer(
    ctx: Context<AddAssetItemOfferContext>,
    asset_index: u32,
    asset_type_index: u32,
    asset_type: u8,
    options: Vec<u8>,
) -> Result<()> {
    msg!("Received index: {}", asset_index);
    msg!("Received asset_type_index: {}", asset_type_index);
    msg!("Received options: {:?}", options);

    let offer = &mut ctx.accounts.offer;
    let asset_item_type = AssetItemType::from_code(asset_type)?;

    msg!("User: {:?}", ctx.accounts.user.key());
    msg!("Trade asset count: {}", offer.asset_count);
    msg!("asset_type: {:?}", asset_type);

    // ! @check
    // |-index should be same as current asset count in trade
    require!(
        offer.asset_count == asset_index,
        SolTradeError::AddAssetItemInvalidIndex
    );

    // ? match by asset type
    match asset_item_type {
        AssetItemType::SOL => {
            if let Some(asset_item_sol) = &mut ctx.accounts.asset_item_sol {
                // ! @check
                // |- asset_type_index should be same as current asset count in trade
                require!(
                    offer.asset_sol_count == asset_type_index,
                    SolTradeError::AddAssetItemInvalidIndex
                );

                // ? Deserialize the options
                let options: OptionsAssetItemSOL = OptionsAssetItemSOL::try_from_slice(&options)
                    .map_err(|_| SolTradeError::AddAssetItemInvalidOptions)?;
                msg!("Options: {:?}", options.amount);
                msg!("AssetItemSOL: {:?}", asset_item_sol.amount);

                // ? init
                asset_item_sol.index = asset_index;
                asset_item_sol.type_index = asset_type_index;
                asset_item_sol.from = offer.key();
                asset_item_sol.user = ctx.accounts.user.key();
                asset_item_sol.asset_type = asset_type;
                asset_item_sol.amount = options.amount;

                // ? increase the trade asset count
                offer.asset_sol_count = offer.asset_sol_count.checked_add(1).unwrap();
            }
        }
        AssetItemType::SPL => {
            if let Some(asset_item_spl) = &mut ctx.accounts.asset_item_spl {
                // ! @check
                // |- asset_type_index should be same as current asset count in trade
                require!(
                    offer.asset_spl_count == asset_type_index,
                    SolTradeError::AddAssetItemInvalidIndex
                );

                // ? Deserialize the options
                let options: OptionsAssetItemSPL = OptionsAssetItemSPL::try_from_slice(&options)
                    .map_err(|_| SolTradeError::AddAssetItemInvalidOptions)?;
                msg!("Options: {:?}", options.token);
                msg!("AssetItemSPL: {:?}", asset_item_spl.token);

                // ? init
                asset_item_spl.index = asset_index;
                asset_item_spl.type_index = asset_type_index;
                asset_item_spl.from = offer.key();
                asset_item_spl.user = ctx.accounts.user.key();
                asset_item_spl.asset_type = asset_type;
                asset_item_spl.token = Pubkey::from_str(&options.token).unwrap();
                asset_item_spl.mint = Pubkey::from_str(&options.mint).unwrap();

                // ? increase the trade asset count
                offer.asset_spl_count = offer.asset_spl_count.checked_add(1).unwrap();
            }
        }
        _ => {
            return Err(SolTradeError::InvalidAssetItemType.into());
        }
    }

    // ? Update trade asset count
    offer.asset_count = offer.asset_count.checked_add(1).unwrap();
    msg!("Updated trade asset count: {}", offer.asset_count);

    Ok(())
}
