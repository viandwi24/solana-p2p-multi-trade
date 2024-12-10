use {crate::error::SolTradeError, anchor_lang::prelude::*};

#[account]
pub struct Authority {
    pub user: Pubkey,     // 32
    pub trade_count: u32, // 4
}
// size = 8 + (32) + (4)
impl Authority {
    pub const SIZE: usize = 44;
    pub const PREFIX: &'static str = "authority";
}

// ===================================================
// =====================[ ASSET ]=====================
// ===================================================
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AssetItemType {
    SOL,
    SPL,
    CNFT,
}
impl AssetItemType {
    pub fn from_code(code: u8) -> Result<AssetItemType> {
        match code {
            1 => Ok(AssetItemType::SOL),
            2 => Ok(AssetItemType::SPL),
            3 => Ok(AssetItemType::CNFT),
            unknown_code => {
                msg!("Unknow AssetItemType : {}", unknown_code);
                Err(SolTradeError::InvalidAssetItemType.into())
            }
        }
    }
    pub fn to_code(&self) -> u8 {
        match self {
            AssetItemType::SOL => 1,
            AssetItemType::SPL => 2,
            AssetItemType::CNFT => 3,
        }
    }
}
// #[account]
// pub struct BaseAssetItem {
//     pub index: u32,     // 4
//     pub from: Pubkey,   // 32 -> pubkey, and it can be account address of trade or offer
//     pub user: Pubkey,   // 32
//     pub asset_type: u8, // 1 -> define base asset type
// }
// impl BaseAssetItem {
//     pub const PREFIX: &'static str = "asset_item";
// }

#[account]
#[derive(Copy)]
pub struct AssetItemSOL {
    pub index: u32,      // 4
    pub type_index: u32, // 4
    pub from: Pubkey,    // 32 -> pubkey, and it can be account address of trade or offer
    pub user: Pubkey,    // 32
    pub asset_type: u8,  // 1 -> define base asset type
    pub exchanged: bool, // 1
    pub amount: u64,     // 8
}
// size = 8 + (4) + (4) + (32) + (32) + (1) + (1) + (8)
impl AssetItemSOL {
    pub const SIZE: usize = 90;
    pub const PREFIX: &'static str = "asset_item_sol";
}
#[account]
pub struct AssetItemSPL {
    pub index: u32,      // 4
    pub type_index: u32, // 4
    pub from: Pubkey,    // 32 -> pubkey, and it can be account address of trade or offer
    pub user: Pubkey,    // 32
    pub asset_type: u8,  // 1 -> define base asset type
    pub exchanged: bool, // 1
    pub token: Pubkey,   // 32
    pub mint: Pubkey,    // 32
}
// size = 8 + (4) + (4) + (32) + (32) + (1) + (1) + (32) + (32)
impl AssetItemSPL {
    pub const SIZE: usize = 146;
    pub const PREFIX: &'static str = "asset_item_spl";
}

// ===================================================
// =====================[ TRADE ]=====================
// ===================================================
#[account]
pub struct Trade {
    pub index: u32,       // 4
    pub user: Pubkey,     // 32
    pub offer_count: u32, // 4
    pub asset_count: u32, // 4

    // offers
    pub accepted_offer: Pubkey, // 32
    pub exchanged_count: u32,   // 4

    // opts
    pub enabled: bool, // 1

    // allowed users, array pubkey support 5 users, per pubkey is 32 bytes, and vec have 4 bytes + len
    pub allowed_users: Vec<Pubkey>, // 4 + (32 * 5)

    // assets items
    pub asset_sol_count: u32, // 4
    pub asset_spl_count: u32, // 4
}
// size = 8 + (4) + (32) + (4) + (4) + (32) + (4) + (1) + (4 + (32 * 5)) + (4) + (4)
impl Trade {
    pub const SIZE: usize = 261;
    pub const PREFIX: &'static str = "trade";
}

// ===================================================
// =====================[ OFFER ]=====================
// ===================================================
#[account]
pub struct Offer {
    pub index: u32,       // 4
    pub user: Pubkey,     // 32
    pub asset_count: u32, // 4

    // opts
    pub enabled: bool,        // 1
    pub accepted: bool,       // 1
    pub exchanged_count: u32, // 4

    // assets items
    pub asset_sol_count: u32, // 4
    pub asset_spl_count: u32, // 4
}
// size = 8 + (4) + (32) + (4) + (1) + (1) + (4) + (4) + (4)
impl Offer {
    pub const SIZE: usize = 62;
    pub const PREFIX: &'static str = "offer";
}
