use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, MintTo, Burn, Token, TokenAccount, Mint},
    associated_token::AssociatedToken,
};

use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct TransferWithFee<'info> {
    /// Customer paying SOL and spending/earning loyalty tokens
    #[account(mut)]
    pub sender: Signer<'info>,

    /// PDA that is the mint authority for the SPI token
    #[account(
        seeds = [b"spi_mint_authority"],
        bump,
    )]
    /// CHECK: This is the SPI mint authority PDA account validated by seeds constraint
    pub spi_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub spi_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = spi_mint,
        associated_token::authority = sender,
    )]
    pub sender_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub merchant: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn transfer_amount(
    ctx: Context<TransferWithFee>,
    amount: u64,
    mint_amount: u64,
    percentage_off: u8,
) -> Result<()> {

    let user_balance = ctx.accounts.sender_ata.amount;

    let authority_seeds = &[b"spi_mint_authority".as_ref(), &[ctx.bumps.spi_authority]];
    let signer_seeds = &[&authority_seeds[..]];

    let burn_amount = user_balance
        .checked_mul(percentage_off as u64)
        .unwrap()
        .checked_div(100)
        .unwrap();

    if burn_amount > 0 {
        let cpi_accounts = Burn {
            mint: ctx.accounts.spi_mint.to_account_info(),
            from: ctx.accounts.sender_ata.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );

        token::burn(cpi_ctx, burn_amount)?;
    }

    let transfer_accounts = Transfer {
        from: ctx.accounts.sender.to_account_info(),
        to: ctx.accounts.merchant.to_account_info(),
    };

    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_accounts,
    );

    transfer(transfer_ctx, amount)?;

    let cpi_accounts_mint = MintTo {
        mint: ctx.accounts.spi_mint.to_account_info(),
        to: ctx.accounts.sender_ata.to_account_info(),
        authority: ctx.accounts.spi_authority.to_account_info(),
    };

    let cpi_ctx_mint = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts_mint,
        signer_seeds,
    );

    token::mint_to(cpi_ctx_mint, mint_amount)?;

    Ok(())
}
