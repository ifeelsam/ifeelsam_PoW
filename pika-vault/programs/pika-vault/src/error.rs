use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
}

#[error_code]
pub enum MarketplaceError {
    #[msg("input name is too long!")]
    NameTooLong,
    #[msg("failed to verify seller")]
    Verify,
}

#[error_code]
pub enum ListingError {
    #[msg("The listing is not active.")]
    ListingNotActive,

    #[msg("Insufficient funds to complete the purchase.")]
    InsufficientFunds,

    #[msg("Escrow creation failed.")]
    EscrowCreationFailed,

    #[msg("listing is not currently marked as sold")]
    ListingNotSold,

    #[msg("you are not authorized to request a refund for this purchase")]
    UnauthorizedRefund,

    #[msg("escrow funds have already been released")]
    EscrowAlreadyReleased,
}
