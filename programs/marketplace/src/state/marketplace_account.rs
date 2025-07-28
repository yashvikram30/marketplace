use anchor_lang::{prelude::*};

#[account]
#[derive(InitSpace)]
pub struct MarketplaceAccount{

    pub authority: Pubkey, // this is the pubkey of the authority of the marketplace
    pub fee_basis_points: u16, // the fee basis points for the royality to fee_basis_points
    #[max_len(100)]
    pub trusted_collections: Vec<Pubkey>, // a vector containing pubkeys of the collections approved by this marketplace
    pub marketplace_bump: u8 
}