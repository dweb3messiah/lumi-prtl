use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};
use anchor_lang::solana_program::account_info::AccountInfo;



use crate::state::Shipment;

use crate::state::Escrow;


#[derive(Accounts)]
#[instruction(title: String)]
pub struct Ship<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mint::token_program = token_program,
    )]
    pub mint_usd: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_usd,
        associated_token::authority = seller
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>, // Seller's SPL associated token account
    #[account(
        init,
        payer = seller,
        space = 8 + Shipment::INIT_SPACE,
        seeds = [title.as_bytes(), seller.key().as_ref()],
        bump
    )]
    pub shipment: Account<'info, Shipment>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl <'info> Ship<'info> {
    pub fn init_shipment
    (
        &mut self, title: String, 
        //description: String,
        //destination_location: String,
        //current_location: String,
        //destination_coordinates: f64,
        //current_location_coordinates: f64,
        //status: String,
    ) 
    -> 
    Result<()> 
    {
        self.shipment.title = title;
        self.shipment.description = "".to_string();
        self.shipment.destination_location = "".to_string();
        self.shipment.current_location = "".to_string();
        self.shipment.destination_coordinates = 0.0;
        self.shipment.current_location_coordinates = 0.0;
        self.shipment.status = "In Transit".to_string(); // the shipment status is set to "In Transit" by default, because it can only be in transit, delivered or cancelled
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct PaymentForTransportAndTracking<'info> {
    #[account(mut)]
    pub seller: Signer<'info>, 
    #[account(mut)]
    pub logistics: SystemAccount<'info>, // logistics account
    pub mint_usd: InterfaceAccount<'info, Mint>, // mint of the token to be transferred by the seller

    #[account(
        mut,
        associated_token::mint = mint_usd,
        associated_token::authority = seller
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>, // Seller's SPL associated token account
    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint_usd,
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
          mint: self.mint_usd.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
        transfer_checked(cpi_ctx, amount, self.mint_usd.decimals)?;
    
        Ok(()) // Return Ok if the transfer is successful
      }
    
}

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account()]
    pub buyer: SystemAccount<'info>,
    pub mint_usd: InterfaceAccount<'info, Mint>,
    #[account(
      mut,
      associated_token::mint = mint_usd,
      associated_token::authority = seller,
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>,
    pub shipment: Account<'info, Shipment>,
    #[account(
      mut, 
      close = seller,
      has_one = mint_usd,
      has_one = buyer,
      seeds = [b"escrow", escrow.buyer.to_bytes().as_ref(), seeds.to_le_bytes().as_ref()],
      bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
      mut,
      associated_token::mint = mint_usd,
      associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl <'info>Withdraw<'info> {
    pub fn seller_withdrawal(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.seller_ata.to_account_info(),
            mint: self.mint_usd.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let seed_binding = self.escrow.seed.to_le_bytes();
        let buyer_binding = self.escrow.buyer.to_bytes();

        let seeds: [&[u8]; 4] = [
            b"escrow",
            &seed_binding,
            &buyer_binding,
            &[self.escrow.bump],
        ];

        let signer_seeds: &[&[&[u8]]] = &[&seeds];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

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
          let buyer_binding = self.escrow.buyer.to_bytes();
  
          let seeds: [&[u8]; 4] = [
              b"escrow",
              &seed_binding,
              &buyer_binding,
              &[self.escrow.bump],
          ];
  
          let signer_seeds: &[&[&[u8]]] = &[&seeds];
  
          let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  
          close_account(cpi_ctx)?;
  
        Ok(())
      }
  
}
