use anchor_lang::prelude::*;


use crate::state::Shipment;

#[derive(Accounts)]
#[instruction(title: String)]

// logistics account will update the shipment account which was initialized by the seller
pub struct UpdateShipment<'info> {
    #[account(mut)]
    pub logistics: Signer<'info>,
    #[account(
        mut,
        seeds = [title.as_bytes(), logistics.key().as_ref()],
        bump,
        realloc = 8 + Shipment::INIT_SPACE,
        realloc::payer = logistics,
        realloc::zero = true, // realloc::zero means that the account will be zeroed out before the reallocation
      )]
    pub shipment: Account<'info, Shipment>,
    pub system_program: Program<'info, System>,
}

impl <'info> UpdateShipment<'info> {
    pub fn update_shipment(&mut self, title: String, description: String, destination_location: String, current_location: String, destination_coordinates: f64, current_location_coordinates: f64, status: String) -> Result<()> {
        self.shipment.title = title;
        self.shipment.description = description;
        self.shipment.destination_location = destination_location;
        self.shipment.current_location = current_location;
        self.shipment.destination_coordinates = destination_coordinates;
        self.shipment.current_location_coordinates = current_location_coordinates;
        self.shipment.status = status;

        Ok(())
    }
    
}