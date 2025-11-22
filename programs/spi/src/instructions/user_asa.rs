use crate::USER_ASA_SEED;
use crate::error::ErrorCode;
use crate::state::UserASA;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUserASAAccounts<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    /// CHECK: The customer account is verified through the PDA seeds derivation
    customer: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + UserASA::INIT_SPACE,
        seeds = [USER_ASA_SEED, customer.key().as_ref()],
        bump
    )]
    user_asa: Account<'info, UserASA>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUserASA<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    /// CHECK: The customer account is verified through the PDA seeds derivation
    customer: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [USER_ASA_SEED, customer.key().as_ref()],
        bump
    )]
    user_asa: Account<'info, UserASA>,
    system_program: Program<'info, System>,
}

pub fn user_asa(
    ctx: Context<CreateUserASAAccounts>,
    name: String,
    merkle_proof: Vec<[u8; 32]>,
    valid_till_unix_timestamp: u64,
) -> Result<()> {
    let user_asa = &mut ctx.accounts.user_asa;
    require!(name.len() <= 20, ErrorCode::NameTooLong);
    user_asa.name = name;
    user_asa.is_valid = true;
    user_asa.spi_tokens = 0;
    user_asa.total_cashback = 0;
    user_asa.valid_till_unix_timestamp = valid_till_unix_timestamp;
    user_asa.join_date_unix_timestamp = Clock::get()?.unix_timestamp as u64;
    user_asa.total_spent = 0;
    user_asa.total_transactions = 0;
    user_asa.merkle_proof = merkle_proof;

    msg!(
        "UserASA account created successfully for customer: {:?}",
        ctx.accounts.customer.key()
    );
    msg!(
        "   Paid for by authority: {:?}",
        ctx.accounts.authority.key()
    );
    Ok(())
}

pub fn update_user_asa(
    ctx: Context<UpdateUserASA>,
    spi_tokens: Option<u64>,
    is_valid: Option<bool>,
    total_cashback: Option<u64>,
    total_spent: Option<u64>,
    total_transactions: Option<u64>,
    valid_till_unix_timestamp: Option<u64>,
) -> Result<()> {
    let user_asa = &mut ctx.accounts.user_asa;

    if let Some(tokens) = spi_tokens {
        user_asa.spi_tokens = tokens;
    }
    if let Some(is_valid) = is_valid {
        user_asa.is_valid = is_valid;
    }
    if let Some(cashback) = total_cashback {
        user_asa.total_cashback = cashback;
    }
    if let Some(spent) = total_spent {
        user_asa.total_spent = spent;
    }
    if let Some(transactions) = total_transactions {
        user_asa.total_transactions = transactions;
    }
    if let Some(valid_till) = valid_till_unix_timestamp {
        user_asa.valid_till_unix_timestamp = valid_till;
    }

    msg!(
        "UserASA account updated successfully for customer: {:?}",
        ctx.accounts.customer.key()
    );
    Ok(())
}
