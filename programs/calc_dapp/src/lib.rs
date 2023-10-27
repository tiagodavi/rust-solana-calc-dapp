use anchor_lang::prelude::*;

declare_id!("DjuK45k5jZr1nd242qEiaeaRmZXGMTdaGdWZsrZLqeMo");

#[program]
pub mod calc_dapp {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;

        Ok(())
    }
    pub fn add(ctx: Context<Addition>, x: i64, y: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = x + y;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}
