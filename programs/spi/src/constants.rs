use anchor_lang::prelude::*;

// PDA Seeds
#[constant]
pub const USER_ASA_SEED: &[u8] = b"user_asa_spi_trial_23";

#[constant]
pub const MEMBERSHIP_ROOT_SEED: &[u8] = b"membership_root_spi_trial_23";

#[constant]
pub const SPI_MINT_AUTHORITY_SEED: &[u8] = b"spi_mint_authority";

#[constant]
pub const METADATA_SEED: &[u8] = b"metadata";

// Token Configuration
#[constant]
pub const SPI_TOKEN_DECIMALS: u8 = 1;
