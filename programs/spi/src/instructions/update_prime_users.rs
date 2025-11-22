use crate::error::ErrorCode;
use crate::state::CreatePrimeUsersMerkleTreePDA;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateMembershipRoot<'info> {
    #[account(mut, seeds = [b"membership_root_spi_trial_1"], bump)]
    pub membership_root: Account<'info, CreatePrimeUsersMerkleTreePDA>,

    pub admin: Signer<'info>,
}

pub fn update_membership_root(
    ctx: Context<UpdateMembershipRoot>,
    new_merkle_root: [u8; 32],
) -> Result<()> {
    let membership = &mut ctx.accounts.membership_root;

    require!(
        ctx.accounts.admin.key() == membership.authority,
        ErrorCode::Unauthorized
    );

    membership.merkle_root = new_merkle_root;
    Ok(())
}
