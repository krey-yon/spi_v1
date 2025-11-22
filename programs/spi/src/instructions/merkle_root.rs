use anchor_lang::prelude::*;
use crate::{MEMBERSHIP_ROOT_SEED, state::CreatePrimeUsersMerkleTreePDA};

#[derive(Accounts)]
pub struct CreateMembershipRoot<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + CreatePrimeUsersMerkleTreePDA::INIT_SPACE, 
        seeds = [MEMBERSHIP_ROOT_SEED], 
        bump
    )]
    pub membership_root: Account<'info, CreatePrimeUsersMerkleTreePDA>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_membership_root(
    ctx: Context<CreateMembershipRoot>,
    merkle_root: [u8; 32],
) -> Result<()> {
    let membership = &mut ctx.accounts.membership_root;
    membership.authority = ctx.accounts.admin.key();
    membership.merkle_root = merkle_root;
    Ok(())
}
