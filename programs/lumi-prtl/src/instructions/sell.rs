use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount},
};


//use crate::state::Escrow; // Removed unused import

use crate::state::Shipment;


//use super::logistics;

//pub mod logistics;
//pub use logistics::*;


#[derive(Accounts)]
#[instruction(title: String)]
pub struct InitShipment<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        init,
        payer = seller,
        space = 8 + Shipment::INIT_SPACE,
        seeds = [title.as_bytes(), seller.key().as_ref()],
        bump
    )]
    pub shipment: Account<'info, Shipment>,
    pub system_program: Program<'info, System>,
}

impl <'info> InitShipment<'info> {
    pub fn init_shipment(&mut self, title: String) -> Result<()> {
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

    pub mint_b: InterfaceAccount<'info, Mint>, // mint of the token to be transferred by the seller

   // pub mint_c: InterfaceAccount<'info, Mint>, // mint of the token to be received by the logistics account

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = seller
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>, // Seller's SPL associated token account

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = logistics // Logistics account, correct?
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

/*use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount},
};

use crate::state::Shipment;


#[derive(Accounts)]
#[instruction(title: String)]
pub struct InitShipment<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        payer = seller,
        space = 8 + Shipment::INIT_SPACE,
        seeds = [title.as_bytes(), seller.key().as_ref()],
        bump
    )]
    pub shipment: Account<'info, Shipment>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct PaymentForTransportAndTracking<'info> {
    #[account(mut)]
    pub seller: Signer<'info>, 

    pub mint_usdc: InterfaceAccount<'info, Mint>, // Mint of the token to be transferred by the seller and received by the logistics account

    #[account(
        mut,
        associated_token::mint = mint_usdc,
        associated_token::authority = seller
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>, // Seller's associated token account for mint_b

    #[account(
        mut,
        associated_token::mint = mint_usdc,
        associated_token::authority = logistics // Corrected authority reference
    )]
    pub logistics_ata: InterfaceAccount<'info, TokenAccount>, // Logistics provider's associated token account

    pub logistics: Signer<'info>, // Added logistics signer to confirm receipt of tokens

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>, // Token program for transfers
    pub system_program: Program<'info, System>,
}

impl <'info> PaymentForTransportAndTracking<'info> {
    
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        // Transfer the tokens from the seller's account to the logistics account
        let cpi_accounts = TransferChecked {
            from: self.seller_ata.to_account_info(), 
            to: self.logistics_ata.to_account_info(), 
            authority: self.seller.to_account_info(),
            mint: self.mint_usdc.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint_usdc.decimals)?;

        Ok(()) // Return Ok if the transfer is successful
    }
}*/
