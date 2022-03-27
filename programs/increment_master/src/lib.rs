use anchor_lang::{
    prelude::*,
};

use increment::cpi::accounts::IncCounter;
use increment::program::Increment;
use increment::{self, User};


declare_id!("HfzxysBgzWNFrV8Rtef572fSwsRZWXSssp3QmBEN9WwE");

#[program]
pub mod increment_master {
    use super::*;

    pub fn increment_counter(ctx: Context<IncrementCounter>, bump: u8) -> Result<()> {
        let bump = &[bump][..];
        increment::cpi::increment_acc(
            ctx.accounts.increment_counter().with_signer(&[&[bump][..]])
        )
    }


}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
    pub increment_program: Program<'info, Increment>,
    ///CHECK: only used as a signing PDA
    pub authority: UncheckedAccount<'info>
}

impl<'info> IncrementCounter<'info> {
    pub fn increment_counter(&self) -> CpiContext<'_, '_, '_, 'info, IncCounter<'info>> {
        let cpi_program = self.increment_program.to_account_info();
        let cpi_accounts = IncCounter {
            user: self.user.to_account_info(),
            signer: self.authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}