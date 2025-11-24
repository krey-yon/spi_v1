use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct Membership<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut)]
    pub merchant: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn purchase_membership<'info>(ctx: Context<Membership>, amount: u64) -> Result<()> {
    let transfer_accounts = Transfer {
        from: ctx.accounts.sender.to_account_info(),
        to: ctx.accounts.merchant.to_account_info(),
    };

    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_accounts,
    );

    transfer(transfer_ctx, amount)?;
    Ok(())
}
