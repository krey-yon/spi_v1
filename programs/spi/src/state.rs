use anchor_lang::prelude::*;
use anchor_lang::{prelude::Pubkey, InitSpace};

#[account]
#[derive(InitSpace)]
pub struct CreatePrimeUsersMerkleTreePDA {
    pub authority: Pubkey,
    pub merkle_root: [u8; 32],
}

#[account]
#[derive(InitSpace)]
pub struct UserASA {
    #[max_len(30)]
    pub name: String,
    
    pub spi_tokens: u64,
    pub is_valid: bool,
    pub total_cashback: u64,
    pub valid_till_unix_timestamp: u64,
    pub join_date_unix_timestamp: u64,
    pub total_spent: u64,
    pub total_transactions: u64,
    
    #[max_len(20)]
    pub merkle_proof: Vec<[u8; 32]>,
}
