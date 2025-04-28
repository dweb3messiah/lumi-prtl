use anchor_lang::prelude::*;

declare_id!("CZJCDNrgf5GAPJsjm2MeME8MwbV3oFDyuV74oQnbtdMv");

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BuyBumps {
    pub escrow_bump: u8,
}


#[program]
pub mod lumi_prtl {
    use super::*;

    // 1️⃣ Step 1: Buyer begins the process
    pub fn buy(ctx: Context<Buy>, seed: u64, amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, &ctx.bumps)?;
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    // 2️⃣ Step 2: Logistics creates shipment
    pub fn init_shipment(
        ctx: Context<Ship>, 
        title: String,
        //description: String,
        //destination_location: String,
        //current_location: String,
        //destination_coordinates: f64,
        //current_location_coordinates: f64,
        //status: String,
    ) 
    -> Result<()> {
        ctx.accounts.init_shipment(
            title, 
            //description, 
            //destination_location, 
            //current_location,
            //destination_coordinates,
            //current_location_coordinates,
            //status,
        )?;
        Ok(())
    }

    // 3️⃣ Step 3: Buyer pays logistics for transport
    pub fn deposit(ctx: Context<PaymentForTransportAndTracking>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    // 4️⃣ Step 4: Logistics updates shipment in transit
    pub fn update_shipment(
        ctx: Context<UpdateShipment>,
        title: String,
        description: String,
        destination_location: String,
        current_location: String,
        destination_coordinates: f64,
        current_location_coordinates: f64,
        status: String,
    ) -> Result<()> {
        ctx.accounts.update_shipment(
            title,
            description,
            destination_location,
            current_location,
            destination_coordinates,
            current_location_coordinates,
            status,
        )?;
        Ok(())
    }

    // 5️⃣ Step 5 (Optional): Buyer files dispute
    pub fn dispute(
        ctx: Context<Dispute>,
        title: String,
        reason: String,
    ) -> Result<()> {
        require!(!reason.trim().is_empty(), ErrorCode::EmptyDisputeReason);
        ctx.accounts.shipment.title = title;
        ctx.accounts.shipment.status = "DISPUTED".to_string();
        ctx.accounts.shipment.logistics = ctx.accounts.logistics.key();
        ctx.accounts.escrow.is_disputed = true;
        ctx.accounts.escrow.dispute_reason = reason;
        Ok(())
    }

    // 6️⃣ Step 6: Refund buyer (if needed)
    pub fn refund_buyer(ctx: Context<Refund>) -> Result<()> {
        let shipment = &ctx.accounts.shipment;
        require!(
            shipment.status == "LOST" || shipment.status == "DISPUTED",
            DisputeError::ShipmentNotEligibleForRefund
        );
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }

    // 7️⃣ Step 7: Seller withdraws after successful delivery
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let shipment = &ctx.accounts.shipment;
        require!(
            shipment.destination_location == shipment.current_location &&
            shipment.destination_coordinates == shipment.current_location_coordinates,
            CustomError::ShipmentNotArrived
        );
        ctx.accounts.seller_withdrawal()?;
        ctx.accounts.close()?;
        Ok(())
    }
}


#[error_code]
pub enum ErrorCode {
    #[msg("Dispute reason cannot be empty.")]
    EmptyDisputeReason,
}

#[derive(Accounts)]
pub struct Initialize {}
