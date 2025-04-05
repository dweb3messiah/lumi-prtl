use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Shipment {
    #[max_len(32)]
    pub title: String,
    #[max_len(100)]
    pub description: String,
    #[max_len(50)]
    pub destination_location: String,
    #[max_len(50)]
    pub current_location: String,
    pub destination_coordinates: f64,
    pub current_location_coordinates: f64,
    #[max_len(32)]
    pub status: String,
    pub logistics: Pubkey,
    pub bump: u8,
}