use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount},
};
use anchor_lang::solana_program::account_info::AccountInfo;


use crate::state::Shipment;

#[derive(Accounts)]
#[instruction(title: String)]

// logistics account will update the shipment account which was initialized by the seller
pub struct UpdateShipment<'info> {
    pub seller: SystemAccount<'info>,

    #[account(mut)]
    pub logistics: Signer<'info>,
    #[account(
      mint::token_program = token_program,
    )]
    pub mint_usd: InterfaceAccount<'info, Mint>,
    #[account(
      mut, 
      associated_token::mint = mint_usd, 
      associated_token::authority = logistics,
      associated_token::token_program = token_program,
    )]
    pub logistics_ata: InterfaceAccount<'info, TokenAccount>, // Logistics's SPL associated token account
    #[account(
        mut,
        seeds = [title.as_bytes(), seller.key().as_ref()], // this 
        bump,
        realloc = 8 + Shipment::INIT_SPACE,
        realloc::payer = logistics,
        realloc::zero = true, // realloc::zero means that the account will be zeroed out before the reallocation
      )]
    pub shipment: Account<'info, Shipment>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl <'info> UpdateShipment<'info> {
    //pub fn update_shipment(&mut self, title: String, description: String, destination_location: String, current_location: String, destination_coordinates: f64, current_location_coordinates: f64, status: String) -> Result<()> {
    pub fn update_shipment(
      &mut self, 
      title: String,
      description: String,
      destination_location: String,
      current_location: String,
      destination_coordinates: f64,
      current_location_coordinates: f64,
      status: String,
    ) 
    -> Result<()> 
    {
        self.shipment.title = title;
        self.shipment.description = description;
        self.shipment.destination_location = destination_location;
        self.shipment.current_location = current_location;
        self.shipment.destination_coordinates = destination_coordinates;
        self.shipment.current_location_coordinates = current_location_coordinates;
        self.shipment.status = status;
        self.shipment.logistics = self.logistics.key();


        Ok(())
    }    
    
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteShipment <'info> {
  #[account(mut)]
  pub seller: SystemAccount<'info>,
  pub logistics: Signer<'info>,
  #[account(
    mut,
    seeds = [title.as_bytes(), seller.key().as_ref()],
    bump,
    close = logistics,
  )]
  pub shipment: Account<'info, Shipment>,
  pub system_program: Program<'info, System>,
}

impl <'info> DeleteShipment<'info> {
  pub fn delete_shipment_state(&mut self) -> Result<()> {
    Ok(())
  }
}