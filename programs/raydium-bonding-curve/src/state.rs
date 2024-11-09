use crate::consts::INITAL_PRICE_DIVIDER;
use crate::consts::INITIAL_LAMPORTS_FOR_POOL;
use crate::consts::PROPORTION;
use crate::consts::TOKEN_SELL_LIMIT_PERCENT;


use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{Mint};

#[account]
pub struct CurveConfiguration {
    pub fees: u64,
}

impl CurveConfiguration {
    
}

pub struct LiquidityProvider {

}

impl LiquidityProvider {
    
}

#[account]
pub struct LiquidityPool {
    pub creator: Pubkey,
    pub token: Pubkey,
    pub total_supply: u64,
    pub reserve_token: u64,
    pub reserve_token: u64,
    pub bump: u8,
}

impl LiquidityPool {
    
}

