pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2RakX2uLr5Pm3NftQq9X8YatfqHseM8qc5J2DvvaAPDB");

#[program]
pub mod spi {
    use super::*;

    pub fn create_prime_user_merkle_root_pda(
        ctx: Context<CreateMembershipRoot>,
        merkle_root: [u8; 32],
    ) -> Result<()> {
        create_membership_root(ctx, merkle_root)?;
        Ok(())
    }

    pub fn update_prime_user_merkle_tree_pda(
        ctx: Context<UpdateMembershipRoot>,
        new_merkle_root: [u8; 32],
    ) -> Result<()> {
        update_membership_root(ctx, new_merkle_root)?;
        Ok(())
    }

    pub fn create_user_asa(
        ctx: Context<CreateUserASAAccounts>,
        name: String,
        merkle_proof: Vec<[u8; 32]>,
        valid_till_unix_timestamp: u64,
    ) -> Result<()> {
        user_asa(ctx, name, merkle_proof, valid_till_unix_timestamp)?;
        Ok(())
    }

    pub fn update_user_asa_data(
        ctx: Context<UpdateUserASA>,
        is_valid: Option<bool>,
        spi_tokens: Option<u64>,
        total_cashback: Option<u64>,
        total_spent: Option<u64>,
        total_transactions: Option<u64>,
        valid_till_unix_timestamp: Option<u64>,
    ) -> Result<()> {
        update_user_asa(
            ctx,
            spi_tokens,
            is_valid,
            total_cashback,
            total_spent,
            total_transactions,
            valid_till_unix_timestamp,
        )?;
        Ok(())
    }

    pub fn create_token(
        ctx: Context<InitializeSpiTokenMint>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        initialize_spi_token_mint(ctx, name, symbol, uri)?;
        Ok(())
    }

    pub fn main_transfer(
        ctx: Context<TransferWithFee>,
        amount: u64,
        mint_amount: u64,
        percentage_off: u8,
    ) -> Result<()> {
        transfer_amount(ctx, amount, mint_amount, percentage_off)?;
        Ok(())
    }
    
    pub fn membership(ctx: Context<Membership>, amount:u64) -> Result<()>{
        purchase_membership(ctx, amount)?;
        Ok(())
    }
}
