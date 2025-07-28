use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ListingAccount{

    pub seller: Pubkey, // seller pubkey stored
    pub mint: Pubkey, // mint of the token listed
    pub price: u64, // price of the listed token, set by seller
    pub created_at: i64, // time at which the listed token is created
    pub listing_bump: u8, // bump of the listing
    pub escrow_vault_bump: u8, // escrow_vault_bump for every escrow vault created for a nft listing

}