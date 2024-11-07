use anchor_lang::prelude::*;
mod consts;
mod errors;
mod state;

declare_id!("FnVRGapn2YBV7C74H7SyPcYi9Nn3cpc38F2DdEEStTL8");

#[program]
pub mod raydium_bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
