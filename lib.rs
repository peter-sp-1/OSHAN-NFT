use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

declare_id!("2U75pbhcQmfQfYgmC4HELdeN7qixUjmjN3XLHxh4bJA1");

#[program]
pub mod nft_sales {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        metadata_uri: String,
        price: u64,
        bump: u8,
    ) -> Result<()> {
        let nft = &mut ctx.accounts.nft_account;
        nft.owner = ctx.accounts.owner.key();
        nft.metadata_uri = metadata_uri.clone();
        nft.price = price;
        nft.for_sale = true;
        nft.bump = bump;

        msg!("NFT minted with URI: {} and price: {}", metadata_uri, price);
        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNFT>, sale_price: u64) -> Result<()> {
        let nft = &mut ctx.accounts.nft_account;

        // Ensure NFT is for sale and price matches
        require!(nft.for_sale, CustomError::NftNotForSale);
        require!(sale_price == nft.price, CustomError::InvalidPrice);

        // Transfer ownership
        nft.owner = ctx.accounts.buyer.key();
        nft.for_sale = false;

        // Transfer funds
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &ctx.accounts.seller.key(),
            sale_price,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.seller.to_account_info(),
            ],
        )?;

        msg!("NFT sold to: {}", ctx.accounts.buyer.key());
        Ok(())
    }
}

#[account]
pub struct NFT {
    pub owner: Pubkey,
    pub metadata_uri: String,
    pub price: u64,
    pub for_sale: bool,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(
        init, 
        seeds = [b"nft", owner.key().as_ref()],
        bump,
        payer = owner, 
        space = 8 + 32 + 256 + 8 + 8 + 1
    )]
    pub nft_account: Account<'info, NFT>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyNFT<'info> {
    #[account(
        mut,
        seeds = [b"nft", seller.key().as_ref()],
        bump = nft_account.bump
    )]
    pub nft_account: Account<'info, NFT>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    #[msg("NFT is not for sale.")]
    NftNotForSale,
    #[msg("Price does not match the sale price.")]
    InvalidPrice,
}
