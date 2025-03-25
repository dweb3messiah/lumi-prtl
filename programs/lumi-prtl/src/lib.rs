use anchor_lang::prelude::*;

declare_id!("CZJCDNrgf5GAPJsjm2MeME8MwbV3oFDyuV74oQnbtdMv");

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;


#[program]
pub mod lumi_prtl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
