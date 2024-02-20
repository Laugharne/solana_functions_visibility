use anchor_lang::prelude::*;

declare_id!("5gxeL3AFd6utfoUjuRxRHyFbujXEZuUdFonBXNwaas64");

#[program]
pub mod visibility {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
