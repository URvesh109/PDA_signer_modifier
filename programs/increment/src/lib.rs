use anchor_lang::{
    prelude::*,
};

declare_id!("cdiBmZiebfEhTpsdNj3ps7djt8m7kra6pGCf3w21MNo");

#[program]
pub mod increment {

    use super::*;

    pub fn initialize_acc(ctx: Context<InitializeAcc>, count: u64, auth: Pubkey) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.counter = count;
        user.authkey = auth;
        Ok(())
    }

    pub fn increment_acc(ctx: Context<IncCounter>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        if user.authkey.key() != ctx.accounts.signer.key() {
            return err!(MyError::WrongSigner);
        } 
        user.counter += 1;
        Ok(())
    }

}

#[error_code]
pub enum MyError {
    #[msg("Not a expected signature")]
    WrongSigner
}

#[derive(Accounts)]
pub struct InitializeAcc<'info> {
    #[account(init, payer = signer, space = 8+40)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct IncCounter<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
    pub signer: Signer<'info>
}

#[account]
pub struct User {
    pub counter: u64,
    pub authkey: Pubkey
}