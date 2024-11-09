use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod state;
pub mod utils;
pub mod instructions;


declare_id!("FnVRGapn2YBV7C74H7SyPcYi9Nn3cpc38F2DdEEStTL8");

#[program]
pub mod raydium_bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}