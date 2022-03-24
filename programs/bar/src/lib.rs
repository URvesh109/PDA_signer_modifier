use anchor_lang::prelude::*;


declare_id!("54XffRfKduCPwnq5zRpQJBk5BsLo5kBaqZAV2CMvtjKV");

#[program]
pub mod bar {
    use super::*;

    pub fn set_data(ctx: Context<Initialize>, data: String) -> Result<()> {
        let ref mut user = ctx.accounts.user;
        user.name = data;
        Ok(())
    }

    pub fn change_name(ctx: Context<ChangeName>, name: String) -> Result<()> {
        let ref mut user = ctx.accounts.user;
        if name.as_bytes().len() > 200 {
            panic!();
        }

        user.name = name;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=authority, space=8+4+200)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ChangeName<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
    pub signer: Signer<'info>

}

#[account]
pub struct User {
    pub name: String
}