use anchor_lang::prelude::*;

declare_id!("39dR8Kz9feCALZPKLVFQaozDtRu5QZJBoL7JhwSdqKbk");

#[program]
pub mod foo {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        let ref mut foo = ctx.accounts.foo;
        foo.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8+8)]
    pub foo: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub foo: Account<'info, Data>,
    pub signer: Signer<'info>
}


#[account]
pub struct Data {
    pub data: u64
}