use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount},
};

#[derive(Accounts)]
pub struct PaymentForTransportAndTracking<'info> {
    #[account(mut)]
    pub seller: Signer<'info>, 
    #[account(mut)]
    pub logistics: SystemAccount<'info>, // logistics account
    pub mint_b: InterfaceAccount<'info, Mint>, // mint of the token to be transferred by the seller
   // pub mint_c: InterfaceAccount<'info, Mint>, // mint of the token to be received by the logistics account
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = seller
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>, // Seller's SPL associated token account
    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint_b,
        associated_token::authority = logistics,
    )]
    pub logistics_ata: InterfaceAccount<'info, TokenAccount>, // Receiver's SPL associated token account
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>, // Token program for executing transfers

    pub system_program: Program<'info, System>
}

impl <'info> PaymentForTransportAndTracking<'info> {
    
    pub fn deposit(&mut self, amount: u64) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        // Transfering the tokens from the seller's account to the logistics account
    
        let cpi_accounts = TransferChecked {
          from: self.seller_ata.to_account_info(), 
          to: self.logistics_ata.to_account_info(), 
          authority: self.seller.to_account_info(),
          mint: self.mint_b.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
        transfer_checked(cpi_ctx, amount, self.mint_b.decimals)?;
    
        Ok(()) // Return Ok if the transfer is successful
      }
    
}
