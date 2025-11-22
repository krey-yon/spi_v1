use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to update the membership root.")]
    Unauthorized,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("❌ Name exceeds maximum length allowed.")]
    NameTooLong,
    #[msg("❌ Merkle proof exceeds maximum length allowed.")]
    MerkleProofTooLarge,
}
