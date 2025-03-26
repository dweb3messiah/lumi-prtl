use anchor_lang::prelude::*;

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


