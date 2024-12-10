use anchor_lang::prelude::*;

#[error_code]
pub enum SolTradeError {
    // intructions: create_trade
    #[msg("Too many allowed users")]
    CreateTradeTooManyAllowedUsers,
    #[msg("Not allowed user")]
    CreateTradeNotAllowedUser,

    // instructions: add_asset_item
    #[msg("Invalid Options")]
    AddAssetItemInvalidOptions,
    #[msg("Index provided does not match the index of the asset item")]
    AddAssetItemInvalidIndex,
    #[msg("Invalid Data AssetItem Serialization")]
    AddAssetItemInvalidDataAssetItemSerialization,

    // instructions: create_offer
    #[msg("Index provided does not match the index of the offer")]
    CreateOfferInvalidIndex,

    // state: AssetItemType
    #[msg("Invalid AssetItemType")]
    InvalidAssetItemType,
}
