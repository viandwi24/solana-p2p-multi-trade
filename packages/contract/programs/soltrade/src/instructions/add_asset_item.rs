use {
    crate::{
        error::SolTradeError,
        state::{AssetItemSOL, AssetItemSPL, AssetItemType, Offer, Trade},
    },
    anchor_lang::prelude::*,
    borsh::BorshDeserialize,
    std::str::FromStr,
};

// ======================================================================
// =====================[ ASSET ITEM OPTIONS ]===========================
// ======================================================================
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OptionsAssetItemSOL {
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OptionsAssetItemSPL {
    pub token: String,
    pub mint: String,
}

// ======================================================================
// =======================[ TRADE OR OFFER ]=============================
// ======================================================================
trait TradeOrOffer {
    fn get_asset_count(&self) -> u32;
    fn set_asset_count(&mut self, count: u32);

    fn get_sol_count(&self) -> u32;
    fn set_sol_count(&mut self, count: u32);

    fn get_spl_count(&self) -> u32;
    fn set_spl_count(&mut self, count: u32);

    fn ex(&self);
}
impl<'info> TradeOrOffer for Account<'info, Trade> {
    fn get_asset_count(&self) -> u32 {
        self.asset_count
    }

    fn set_asset_count(&mut self, count: u32) {
        self.asset_count = count;
    }

    fn get_sol_count(&self) -> u32 {
        self.asset_sol_count
    }

    fn set_sol_count(&mut self, count: u32) {
        self.asset_sol_count = count;
    }

    fn get_spl_count(&self) -> u32 {
        self.asset_spl_count
    }

    fn set_spl_count(&mut self, count: u32) {
        self.asset_spl_count = count;
    }

    fn ex(&self) {
        self.exit(&crate::id()).unwrap();
    }
}
impl<'info> TradeOrOffer for Account<'info, Offer> {
    fn get_asset_count(&self) -> u32 {
        self.asset_count
    }

    fn set_asset_count(&mut self, count: u32) {
        self.asset_count = count;
    }

    fn get_sol_count(&self) -> u32 {
        self.asset_sol_count
    }

    fn set_sol_count(&mut self, count: u32) {
        self.asset_sol_count = count;
    }

    fn get_spl_count(&self) -> u32 {
        self.asset_spl_count
    }

    fn set_spl_count(&mut self, count: u32) {
        self.asset_spl_count = count;
    }

    fn ex(&self) {
        self.exit(&crate::id()).unwrap();
    }
}
// fn calculate_asset_item_space(asset_type: u8) -> usize {
//     // Determine the space required for AssetItem based on asset_type
//     match AssetItemType::from_code(asset_type).unwrap() {
//         AssetItemType::SOL => AssetItemSOL::SIZE, // Use the size of AssetItemSOL
//         AssetItemType::SPL => AssetItemSPL::SIZE, // Use the size of AssetItemSPL
//         _ => 0, // Return 0 if invalid type, though we shouldn't reach here due to validation
//     }
// }

// ======================================================================
// ==========================[ CONTEXT ]=================================
// ======================================================================
#[derive(Accounts)]
#[instruction(from_type: u8, _index: u32, asset_type_index: u32, asset_type: u8)]
pub struct AddAssetItemContext<'info> {
    // ? trade: trade or offer account to add asset item
    // #[account(mut, seeds = [Trade::PREFIX.as_bytes()], bump)]
    #[account(mut)]
    /// CHECK: this can be trade or offer and check in runtime
    pub trade_or_offer: AccountInfo<'info>,

    // ? just for struct
    // pub asset_item_base: Option<Account<'info, BaseAssetItem>>,
    #[account(
        init,
        payer = user,
        space = AssetItemSOL::SIZE,
        seeds = [
            trade_or_offer.key().as_ref(),
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
            trade_or_offer.key().as_ref(),
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

pub fn add_asset_item(
    ctx: Context<AddAssetItemContext>,
    from_type: u8,
    asset_index: u32,
    asset_type_index: u32,
    asset_type: u8,
    options: Vec<u8>,
) -> Result<()> {
    msg!("Received from_type: {}", from_type);
    msg!("Received asset_index: {}", asset_index);
    msg!("Received asset_type_index: {}", asset_type_index);
    msg!("Received asset_type: {}", asset_type);
    msg!("Received options: {:?}", options);

    let asset_item_type = AssetItemType::from_code(asset_type)?;

    // ? Check if the trade is owned by the user
    if from_type == 1 {
        let trade: &mut Account<Trade> =
            &mut Account::try_from(&mut ctx.accounts.trade_or_offer.to_account_info())?;
        process_asset_item(
            trade,
            &trade.key(),
            ctx.accounts.trade_or_offer.to_account_info(),
            asset_item_type,
            asset_index,
            asset_type_index,
            asset_type,
            &options,
            &ctx.accounts.user,
            &mut ctx.accounts.asset_item_sol,
            &mut ctx.accounts.asset_item_spl,
        )?;
    } else if from_type == 2 {
        let offer: &mut Account<Offer> =
            &mut Account::try_from(&mut ctx.accounts.trade_or_offer.to_account_info())?;
        process_asset_item(
            offer,
            &offer.key(),
            ctx.accounts.trade_or_offer.to_account_info(),
            asset_item_type,
            asset_index,
            asset_type_index,
            asset_type,
            &options,
            &ctx.accounts.user,
            &mut ctx.accounts.asset_item_sol,
            &mut ctx.accounts.asset_item_spl,
        )?;
    }

    Ok(())
}

fn process_asset_item<'info, T: TradeOrOffer>(
    trade_or_offer: &mut T,
    from_key: &Pubkey,
    from_account_info: AccountInfo<'info>,
    asset_item_type: AssetItemType,
    asset_index: u32,
    asset_type_index: u32,
    asset_type: u8,
    options: &[u8],
    user: &Signer<'info>,
    asset_item_sol: &mut Option<Account<'info, AssetItemSOL>>,
    asset_item_spl: &mut Option<Account<'info, AssetItemSPL>>,
) -> Result<()> {
    match asset_item_type {
        AssetItemType::SOL => {
            if let Some(asset_item_sol) = asset_item_sol {
                // ! check
                // |- check if the asset item index is same as the asset type index
                require!(
                    trade_or_offer.get_sol_count() == asset_type_index,
                    SolTradeError::AddAssetItemInvalidIndex
                );
                msg!("asset_item_sol count: {:?}", trade_or_offer.get_sol_count());

                let options: OptionsAssetItemSOL = OptionsAssetItemSOL::try_from_slice(options)
                    .map_err(|_| SolTradeError::AddAssetItemInvalidOptions)?;

                // debug
                msg!(
                    "asset_item_sol lamports : {:?}",
                    asset_item_sol.to_account_info().lamports()
                );
                msg!("user lamports : {:?}", user.to_account_info().lamports());

                msg!("transfer lamport {:?}", options.amount);
                let ix = anchor_lang::solana_program::system_instruction::transfer(
                    &user.key(),
                    &from_key.key(),
                    // &asset_item_sol.key(),
                    options.amount,
                );
                anchor_lang::solana_program::program::invoke(
                    &ix,
                    &[
                        user.to_account_info(),
                        from_account_info.to_account_info(),
                        // asset_item_sol.to_account_info(),
                    ],
                )?;

                msg!("asset_item_sol init");
                asset_item_sol.index = asset_index;
                asset_item_sol.type_index = asset_type_index;
                asset_item_sol.from = *from_key;
                asset_item_sol.user = user.key();
                asset_item_sol.asset_type = asset_type;
                asset_item_sol.amount = options.amount;

                // // ! transfer lamports
                // **asset_item_sol.to_account_info().try_borrow_mut_lamports()? +=
                //     asset_item_sol.amount;
                // **user.to_account_info().try_borrow_mut_lamports()? -= asset_item_sol.amount;

                // debug
                msg!(
                    "asset_item_sol lamports after : {:?}",
                    asset_item_sol.to_account_info().lamports()
                );
                msg!(
                    "user lamports after : {:?}",
                    user.to_account_info().lamports()
                );

                msg!("asset_item_sol count: {:?}", trade_or_offer.get_sol_count());
                trade_or_offer.ex();
            }
            Ok(())
        }
        AssetItemType::SPL => {
            if let Some(asset_item_spl) = asset_item_spl {
                // ! check
                // |- check if the asset item index is same as the asset type index
                require!(
                    trade_or_offer.get_spl_count() == asset_type_index,
                    SolTradeError::AddAssetItemInvalidIndex
                );
                msg!("asset_item_spl count: {:?}", trade_or_offer.get_spl_count());

                let options: OptionsAssetItemSPL = OptionsAssetItemSPL::try_from_slice(options)
                    .map_err(|_| SolTradeError::AddAssetItemInvalidOptions)?;

                msg!("asset_item_spl init");
                asset_item_spl.index = asset_index;
                asset_item_spl.type_index = asset_type_index;
                asset_item_spl.from = *from_key;
                asset_item_spl.user = user.key();
                asset_item_spl.asset_type = asset_type;
                asset_item_spl.token = Pubkey::from_str(&options.token).unwrap();
                asset_item_spl.mint = Pubkey::from_str(&options.mint).unwrap();

                msg!("asset_item_spl increase");
                trade_or_offer
                    .set_spl_count(trade_or_offer.get_spl_count().checked_add(1).unwrap());
                trade_or_offer
                    .set_asset_count(trade_or_offer.get_asset_count().checked_add(1).unwrap());

                msg!("asset_item_spl count: {:?}", trade_or_offer.get_spl_count());
                trade_or_offer.ex();
            }

            Ok(())
        }
        _ => Err(SolTradeError::InvalidAssetItemType.into()),
    }
}
