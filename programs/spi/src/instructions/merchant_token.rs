use anchor_lang::prelude::*;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata, ID as METADATA_PROGRAM_ID};
use anchor_spl::token::Token;
use anchor_spl::token_interface::{Mint};

use crate::{METADATA_SEED, SPI_MINT_AUTHORITY_SEED, SPI_TOKEN_DECIMALS};

#[derive(Accounts)]
pub struct InitializeSpiTokenMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [SPI_MINT_AUTHORITY_SEED],
        bump,
    )]
    ///CHECK: check for account
    pub spi_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = SPI_TOKEN_DECIMALS,
        mint::authority = spi_authority,
        mint::freeze_authority = spi_authority,
    )]
    pub spi_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Metadata PDA (Metaplex)
    #[account(
        mut,
        seeds = [
            METADATA_SEED,
            METADATA_PROGRAM_ID.as_ref(),
            spi_mint.key().as_ref(),
        ],
        bump,
        seeds::program = METADATA_PROGRAM_ID
    )]
    pub metadata: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub metadata_program: Program<'info, Metadata>,
}

pub fn initialize_spi_token_mint(
    ctx: Context<InitializeSpiTokenMint>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let mint = &ctx.accounts.spi_mint;

    let authority_seeds = &[
        b"spi_mint_authority".as_ref(),
        &[ctx.bumps.spi_authority],
    ];
    let signer_seeds = &[&authority_seeds[..]];

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata:ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.spi_mint.to_account_info(),
                mint_authority: ctx.accounts.spi_authority.to_account_info(),
                update_authority: ctx.accounts.spi_authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false,
        true,
        None,
    )?;
    
    msg!("Token mint created successfully.");
    
    msg!("SPI Loyalty Token mint created: {}", mint.key());
    Ok(())
}
