use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("The price must be greater than 0")]
    InvalidPrice,
}
