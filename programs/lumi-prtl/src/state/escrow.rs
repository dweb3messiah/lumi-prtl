use anchor_lang::prelude::*;

#[account()]
#[derive(InitSpace)]

pub struct Escrow {
    pub seed: u64,
    pub buyer: Pubkey,
    pub mint_usd: Pubkey,
    pub amount: u64, // this is the amount of the token to be transferred in exchange for the goods into the vault 
    pub bump: u8,
    pub is_completed: bool,
    pub is_cancelled: bool,
    pub is_disputed: bool,
    #[max_len(100)]
    pub dispute_reason: String,
    pub is_refunded: bool,
    pub is_dispute_resolved: bool,

}

