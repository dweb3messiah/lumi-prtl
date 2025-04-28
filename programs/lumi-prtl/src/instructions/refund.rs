use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount, CloseAccount, close_account},
};
use anchor_lang::solana_program::account_info::AccountInfo;


use crate::state::Escrow;

use crate::state::Shipment;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub mint_usd: InterfaceAccount<'info, Mint>,
    #[account(
      mut, 
      associated_token::mint = mint_usd,
      associated_token::authority = buyer
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      mut,
      close = buyer,
      seeds = [b"escrow", escrow.buyer.to_bytes().as_ref(), escrow.seed.to_le_bytes().as_ref()],
      bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub shipment: Account<'info, Shipment>,
    #[account(
      init, 
      payer = buyer,
      associated_token::mint = mint_usd,
      associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info>{
  pub fn withdraw(&mut self) -> Result<()>{
    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
      to: self.buyer_ata.to_account_info(),
      from: self.vault.to_account_info(),
      authority: self.buyer.to_account_info(),
      mint: self.mint_usd.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_ctx, self.vault.amount, self.mint_usd.decimals)?;

    Ok(())
  }

  pub fn close(&mut self) -> Result<()> {
    let cpi_program = self.token_program.to_account_info();

      let cpi_accounts = CloseAccount {
          account: self.vault.to_account_info(),
          destination: self.buyer.to_account_info(),
          authority: self.escrow.to_account_info(),
      };

      let seed_binding = self.escrow.seed.to_le_bytes();
      let maker_binding = self.escrow.buyer.to_bytes();

      let seeds: [&[u8]; 4] = [
          b"escrow",
          &seed_binding,
          &maker_binding,
          &[self.escrow.bump],
      ];

      let signer_seeds: &[&[&[u8]]] = &[&seeds];

      let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

      close_account(cpi_ctx)?;

    Ok(())
  }
}