use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("duplicate tokens are not allowed.")]
    DupplicateTokenNotAllowed,

    #[msg("failled to allocate the shared.")]
    FailedToAllocateShared,

    #[msg("failed to deallocate shares.")]
    FailedToDeallocateShares,

    #[msg("insufficient shares.")]
    InsufficientShares,

    #[msg("insufficient funds to swap.")]
    InsufficientFunds,

    #[msg("the amount is invalid to swap.")]
    InvalidAmount,

    #[msg("invalid fee is entered.")]
    InvalidFee,

    #[msg("failed to add the liquidity.")]
    FailedToAddLiquidity,

    #[msg("failed to remove the liquidity.")]
    FailedToRemoveLiquidity,

    #[msg("sold token is not enough to remove the pool.")]
    NotEnoughToRemove,

    #[msg("not a pool creator.")]
    NotCreator,

    #[msg("either overflow or underflow occured.")]
    OverflowOrUnderflowOccured,

    #[msg("token amount is too big to sell.")]
    TokenAmountToSellToBig,

    #[msg("SOL is not enough in the vault.")]
    NotEnoughSolInVault,

    #[msg("token is not enough in the vault.")]
    NotEnoughTokenInVault,

    #[msg("amount is negative.")]
    NegativeNumber,

}