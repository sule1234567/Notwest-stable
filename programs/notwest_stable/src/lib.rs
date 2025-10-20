use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWqCk6uJfC6JrLxvJ3z8nqzQ6");

#[program]
pub mod notwest_stable {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_value: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.value = initial_value;
        msg!("✅ Account initialized with value: {}", initial_value);
        Ok(())
    }

    pub fn deposit(ctx: Context<Update>, amount: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.value += amount;
        msg!("💰 Deposited: {} | New balance: {}", amount, data_account.value);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Update>, amount: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        require!(data_account.value >= amount, ErrorCode::InsufficientFunds);
        data_account.value -= amount;
        msg!("💸 Withdrawn: {} | Remaining balance: {}", amount, data_account.value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
}

#[account]
pub struct DataAccount {
    pub value: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("❌ Not enough funds in account.")]
    InsufficientFunds,
  }
