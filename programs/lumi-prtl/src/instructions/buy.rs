use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount},
};

use crate::state::Escrow;


#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Buy <'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = buyer,
    )]
    pub buyer_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = buyer,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", buyer.key().as_ref(), seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = buyer,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
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
        buyer: self.buyer.key(), // this is the buyer's pubkey
        seller: self.escrow.seller.key(), // this is the seller's pubkey
        bump: bumps.escrow, // this is the bump for the escrow account 
        amount: todo!(), // this is the amount of the token to be transferred in exchange for the goods
        is_completed: todo!(), // this is the status of the escrow account
        is_cancelled: todo!(), 
        is_disputed: todo!(), 
        is_refunded: todo!(),
        is_dispute_resolved: todo!(),
      });
  
      Ok(())
    }
  
    pub fn deposit(&mut self, amount: u64) -> Result<()>{
      let cpi_program = self.token_program.to_account_info();
  
      let cpi_accounts = TransferChecked {
        from: self.buyer_ata_a.to_account_info(), 
        to: self.vault.to_account_info(), 
        authority: self.buyer.to_account_info(),
        mint: self.mint_a.to_account_info(),
      };
  
      let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  
      transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;
  
      Ok(()) 
    } 
} 