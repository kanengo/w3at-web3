use anchor_lang::prelude::*;

declare_id!("4toYn2UYm9EKiwakhDqMk8esdMsoZ72rhwsDJjtuw7CQ");

#[program]
pub mod w3at_web3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, uri: String) -> Result<String> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(uri.to_string())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
