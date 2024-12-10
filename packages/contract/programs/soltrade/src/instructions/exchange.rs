use {
    crate::{
        error::SolTradeError,
        state::{AssetItemSOL, AssetItemSPL, AssetItemType, Offer, Trade},
    },
    anchor_lang::prelude::*,
    solana_program::{
        instruction::AccountMeta,
        program::{invoke, invoke_signed},
        pubkey::Pubkey,
        system_instruction,
    },
};

#[derive(Accounts)]
#[instruction(trade_index: u32, offer_index: u32)]
pub struct ExchangeContext<'info> {
    // ? trade: account to create
    #[account(mut, seeds = [ Trade::PREFIX.as_bytes(), trade_index.to_le_bytes().as_ref() ], bump)]
    pub trade: Account<'info, Trade>,

    // ? offer: account to create
    #[account(mut, seeds = [ trade.key().as_ref(), Offer::PREFIX.as_bytes(), offer_index.to_le_bytes().as_ref() ], bump)]
    pub offer: Account<'info, Offer>,

    #[account(mut)]
    pub asset_item_sol: Option<Account<'info, AssetItemSOL>>,

    #[account(mut)]
    pub asset_item_spl: Option<Account<'info, AssetItemSPL>>,

    // ? user: who the target user
    #[account(mut)]
    /// CHECK: this user is from the trade or offer
    pub user_from: AccountInfo<'info>,

    // ? user: who is creating the trade
    #[account(mut)]
    pub user: Signer<'info>,

    // ? system_program: system program
    pub system_program: Program<'info, System>,
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
    let user = &mut ctx.accounts.user;
    let trade = &mut ctx.accounts.trade;
    let offer = &mut ctx.accounts.offer;

    let asset_item_type = AssetItemType::from_code(asset_type)?;

    msg!("trade: {:?}", trade.key());
    msg!("offer: {:?}", offer.key());

    if from_type == 1 {
        // if from trade, so exchange from trade to offer
        msg!(
            "exchange from trade to offer | from user {:?} to user {:?}",
            ctx.accounts.user.key(),
            ctx.accounts.user_from.key()
        );

        // prepare
        let program_id = ctx.program_id;
        let (pda_trade, bump_trade) = Pubkey::find_program_address(
            &[trade.key().as_ref(), &trade_index.to_le_bytes()],
            program_id,
        );
        let signer_seeds: &[&[&[u8]]] = &[&[b"trade", &[bump_trade]]];

        // process
        process_asset_item(
            from_type,
            &trade.key(),
            trade.to_account_info(),
            signer_seeds,
            &ctx.accounts.user_from.to_account_info(),
            &ctx.accounts.user.to_account_info(),
            asset_item_type,
            &mut ctx.accounts.asset_item_sol,
            &mut ctx.accounts.asset_item_spl,
            ctx.accounts.system_program.to_account_info(),
        )?;

        // update the trade
        trade.exchanged_count = trade.exchanged_count.checked_add(1).unwrap();
    } else if from_type == 2 {
        // if from offer, so exchange from offer to trade
        msg!(
            "exchange from offer to trade | from user {:?} to user {:?}",
            ctx.accounts.user_from.key(),
            ctx.accounts.user.key()
        );

        // prepare
        let program_id = ctx.program_id;
        let (pda_offer, bump_offer) = Pubkey::find_program_address(
            &[
                trade.key().as_ref(),
                Offer::PREFIX.as_bytes(),
                &offer_index.to_le_bytes(),
            ],
            program_id,
        );
        let trade_address = trade.key();
        let offer_index_bytes = offer_index.to_le_bytes();
        let signer_seeds: &[&[&[u8]]] = &[&[
            trade_address.as_ref(),
            b"offer",
            &offer_index_bytes,
            &[bump_offer],
        ]];

        // process
        process_asset_item(
            from_type,
            &offer.key(),
            offer.to_account_info(),
            signer_seeds,
            &ctx.accounts.user_from.to_account_info(),
            &ctx.accounts.user.to_account_info(),
            asset_item_type,
            &mut ctx.accounts.asset_item_sol,
            &mut ctx.accounts.asset_item_spl,
            ctx.accounts.system_program.to_account_info(),
        )?;

        // update the offer
        offer.exchanged_count = offer.exchanged_count.checked_add(1).unwrap();
    } else {
        return Err(SolTradeError::InvalidAssetItemType.into());
    }

    Ok(())
}

fn process_asset_item<'info>(
    from_type: u8,
    from_key: &Pubkey,
    from_account_info: AccountInfo<'info>,
    from_signer: &[&[&[u8]]],

    from_user: &AccountInfo<'info>,
    to_user: &AccountInfo<'info>,

    asset_item_type: AssetItemType,
    asset_item_sol: &mut Option<Account<'info, AssetItemSOL>>,
    asset_item_spl: &mut Option<Account<'info, AssetItemSPL>>,

    system_program: AccountInfo<'info>,
) -> Result<()> {
    match asset_item_type {
        AssetItemType::SOL => {
            if let Some(asset_item_sol) = asset_item_sol {
                // ! @check
                // |- check asset_item_sol.exchanged must not be true
                assert!(!asset_item_sol.exchanged, "Asset item already exchanged");
                // |- check asset_item_sol.from must be from_key
                assert_eq!(asset_item_sol.from, *from_key, "Invalid from key");

                // tranfer sol first
                msg!("transfer lamports {:?}", asset_item_sol.amount);
                msg!(
                    "from_account_info lamports before : {:?}",
                    from_account_info.to_account_info().lamports()
                );
                msg!(
                    "user lamports before : {:?}",
                    to_user.to_account_info().lamports()
                );

                // let ix = anchor_lang::solana_program::system_instruction::transfer(
                //     &from_account_info.key(),
                //     // &asset_item_sol.key(),
                //     &to_user.key(),
                //     asset_item_sol.amount,
                // );
                // anchor_lang::solana_program::program::invoke_signed(
                //     &ix,
                //     &[
                //         // from
                //         from_account_info.to_account_info(),
                //         // asset_item_sol.to_account_info(),
                //         // to
                //         to_user.clone(),
                //     ],
                //     from_signer,
                // )?;
                // ! transfer lamports
                **from_account_info
                    .to_account_info()
                    .try_borrow_mut_lamports()? -= asset_item_sol.amount;
                **to_user.to_account_info().try_borrow_mut_lamports()? += asset_item_sol.amount;

                // debug
                msg!(
                    "from_account_info lamports after : {:?}",
                    from_account_info.to_account_info().lamports()
                );
                msg!(
                    "user lamports after : {:?}",
                    to_user.to_account_info().lamports()
                );

                // update the asset item
                asset_item_sol.exchanged = true;
            }

            Ok(())
        }
        AssetItemType::SPL => Ok(()),
        _ => {
            return Err(SolTradeError::InvalidAssetItemType.into());
        }
    }
}
