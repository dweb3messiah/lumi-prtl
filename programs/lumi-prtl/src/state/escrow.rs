use anchor_lang::prelude::*;

#[account()]
#[derive(InitSpace)]

pub struct Escrow {
    pub seed: u64,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64, // this is the amount of the token to be transferred in exchange for the goods into the vault 
    pub bump: u8,
    pub is_completed: bool,
    pub is_cancelled: bool,
    pub is_disputed: bool,
    pub is_refunded: bool,
    pub is_dispute_resolved: bool,
}

/*
#[account()]
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
}
 */