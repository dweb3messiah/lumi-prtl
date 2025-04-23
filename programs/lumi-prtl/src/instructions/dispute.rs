use anchor_lang::prelude::*;

use crate::{state::{Escrow, Shipment}, FiledDisputeError};

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Dispute<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: We only need the key here to verify shipment
    #[account(mut)]
    pub logistics: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [title.as_bytes(), escrow.buyer.to_bytes().as_ref()],
        bump
    )]
    pub shipment: Account<'info, Shipment>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.buyer.to_bytes().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
}

impl<'info> Dispute<'info> {
    pub fn file_dispute(&mut self, reason: String) -> Result<()> {
        // Disallow disputes if shipment is already delivered
        require!(
            self.shipment.status != "Delivered",
            FiledDisputeError::ShipmentAlreadyDelivered
        );
    
        // Disallow empty reasons
        require!(
            !reason.trim().is_empty(),
            FiledDisputeError::EmptyDisputeReason
        );
    
        self.escrow.is_disputed = true;
        self.escrow.dispute_reason = reason;
    
        Ok(())
    }
    

    pub fn resolve_dispute(&mut self, resolved_by_logistics: bool) -> Result<()> {
        // Only logistics can resolve disputes
        require_keys_eq!(
            self.logistics.key(),
            self.shipment.logistics,
            FiledDisputeError::UnauthorizedResolver
        );

        require!(self.escrow.is_disputed, FiledDisputeError::NoActiveDispute);

        self.escrow.is_disputed = false;
        self.escrow.is_dispute_resolved = resolved_by_logistics;

        Ok(())
    }
}
