use anchor_lang::prelude::*;
use bar::cpi::accounts::{Initialize, ChangeName};
use bar::program::Bar;
use bar::{self, User};
use anchor_lang::solana_program::{
    entrypoint::ProgramResult,
    program::{invoke_signed},
    rent::Rent,
    pubkey::Pubkey,
    system_instruction,
};

const PREFIX: &str = "userdata";

declare_id!("C9gADctaRKygcKSoH6TYuegVewTiF224BrmetWYhDZrj");

#[program]
pub mod bar_master {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, bump: u8, data: String) -> Result<()> {
        let seeds = [PREFIX.as_bytes(), ctx.accounts.signer.key.as_ref(), &[bump]];

        bar::cpi::set_data(
             ctx.accounts.set_data_ctx().with_signer(&[&seeds]),
            data)
    }

    pub fn create_user_invoke(ctx: Context<CreateUser>, bump:u8) -> ProgramResult {

        let seeds = [PREFIX.as_bytes(), ctx.accounts.signer.key.as_ref(), &[bump]];

        invoke_signed(
            &system_instruction::create_account(
                ctx.accounts.signer.key,
                ctx.accounts.userdata.key,
                Rent::get()?.minimum_balance(213),
                213,
                ctx.accounts.probability.key
            ), &[
                ctx.accounts.userdata.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ], &[
                &seeds
            ])

    }

    pub fn change_name(ctx: Context<ChangeUserName>, bump:u8, data: String) -> Result<()> {

        let seeds = [PREFIX.as_bytes(), ctx.accounts.signer.key.as_ref(), &[bump]];

        bar::cpi::change_name(
             ctx.accounts.change_name_ctx().with_signer(&[&seeds]),
            data)
    }
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    /// CHECK: testing purpose
    #[account(mut, seeds=[PREFIX.as_bytes(), signer.key.as_ref()], bump)]
    pub userdata: UncheckedAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub probability: Program<'info, Bar>,
    pub system_program: Program<'info, System>,

}


impl<'info> CreateUser<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Initialize<'info>> {
        let cpi_program = self.probability.to_account_info();
        let cpi_accounts = Initialize {
            user: self.userdata.to_account_info(),
            authority: self.signer.to_account_info(),
            system_program: self.system_program.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct ChangeUserName<'info> {
    #[account(mut)]
    pub userdata: Account<'info, User>,
    pub probability: Program<'info, Bar>,
    /// CHECK: testing purpose
    pub signer: UncheckedAccount<'info>,
}


impl<'info> ChangeUserName<'info> {
    pub fn change_name_ctx(&self) -> CpiContext<'_, '_, '_, 'info, ChangeName<'info>> {
        let cpi_program = self.probability.to_account_info();
        let cpi_accounts = ChangeName {
            user: self.userdata.to_account_info(),
            signer: self.signer.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}