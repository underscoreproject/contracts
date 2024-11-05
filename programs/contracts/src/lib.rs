use anchor_lang::prelude::*;

declare_id!("FJ4vPnyVd2tcyoaJtMXvrMsuPhk9k7GTveCaUaroKyuW");

#[program]
pub mod contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
