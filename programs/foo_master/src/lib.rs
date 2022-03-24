use anchor_lang::prelude::*;
use foo::cpi::accounts::SetData;
use foo::program::Foo;
use foo::{self, Data};

declare_id!("Ffh6UfPxPSkaPH25Md76uzqSvUXBJCW3tTJLBLEX52tS");

#[program]
pub mod foo_master {
    use super::*;

    pub fn pull_strings(ctx: Context<PullStrings>, bump: u8, data: u64) -> Result<()> {
        let bump = &[bump][..];
        foo::cpi::set_data(
            ctx.accounts.set_data_ctx().with_signer(&[&[bump][..]]), 
            data)
    }

    pub fn create_user(ctx: Context<CreateUser>, name: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        if name.as_bytes().len() > 200 {
            panic!()
        }
        user.name = name;
        user.bump = *ctx.bumps.get("user").unwrap();
        Ok(())
    }

    pub fn changer_user_name(ctx: Context<ChangeUserName>, new_name: String) -> Result<()> {
        if new_name.as_bytes().len() > 200 {
            panic!()
        }
        ctx.accounts.user.name = new_name;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub foo: Account<'info, Data>,
    pub foo_program: Program<'info, Foo>,
    ///CHECK: only used as a signing PDA
    pub authority: UncheckedAccount<'info>
}


impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.foo_program.to_account_info();
        let cpi_accounts = SetData {
            foo: self.foo.to_account_info(),
            signer: self.authority.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = signer, space = 8+4+200+1, seeds=[b"user", signer.key().as_ref()], bump)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ChangeUserName<'info> {
    #[account(mut, seeds = [b"user", signer.key().as_ref()], bump=user.bump)]
    pub user: Account<'info, User>,
    pub signer: Signer<'info>
}

#[account]
pub struct User {
    pub name: String,
    pub bump: u8
}