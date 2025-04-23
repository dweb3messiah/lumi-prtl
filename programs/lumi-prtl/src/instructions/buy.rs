use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount},
};
use anchor_lang::solana_program::account_info::AccountInfo;


use crate::state::Escrow;


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Buy <'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
      mint::token_program = token_program,
    )]
    pub mint_usd: InterfaceAccount<'info, Mint>,

    #[account(
      mut, 
      associated_token::mint = mint_usd, 
      associated_token::authority = buyer,
      associated_token::token_program = token_program,
    )]
    pub buyer_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
      init,
      payer = buyer,
      space = 8 + Escrow::INIT_SPACE,
      seeds = [b"escrow", buyer.key().as_ref(), seed.to_le_bytes().as_ref()],
      bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
      init, 
      payer = buyer,
      associated_token::mint = mint_usd,
      associated_token::authority = escrow,
      associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Buy<'info>{
    pub fn init_escrow(&mut self, seed: u64, bumps: &BuyBumps) -> Result<()> {
      self.escrow.set_inner(Escrow {
        seed,
        buyer: self.buyer.key(),
        mint_usd: self.mint_usd.key(),
        bump: bumps.escrow,
        amount: 0,
        is_completed: false,
        is_cancelled: false,
        is_disputed: false,
        dispute_reason: "".to_string(),
        is_refunded: false,
        is_dispute_resolved: false,
      });      
      Ok(())
    }
  
    pub fn deposit(&mut self, amount: u64) -> Result<()>{
      let cpi_program = self.token_program.to_account_info();
  
      let cpi_accounts = TransferChecked {
        from: self.buyer_ata_a.to_account_info(), 
        to: self.vault.to_account_info(), 
        authority: self.buyer.to_account_info(),
        mint: self.mint_usd.to_account_info(),
      };
  
      let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  
      transfer_checked(cpi_ctx, amount, self.mint_usd.decimals)?;
  
      Ok(()) 
    } 
} 