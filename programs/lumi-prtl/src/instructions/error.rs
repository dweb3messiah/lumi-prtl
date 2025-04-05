use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Shipment has not yet arrived at the destination yet.")]
    ShipmentNotArrived,
}

#[error_code]
pub enum DisputeError {
    #[msg("Shipment has not yet arrived at the destination.")]
    ShipmentNotArrived,

    #[msg("Shipment not eligible for refund.")]
    ShipmentNotEligibleForRefund,
}

#[error_code]
pub enum FiledDisputeError {
    #[msg("This shipment has already been delivered.")]
    ShipmentAlreadyDelivered,

    #[msg("You are not authorized to resolve this dispute.")]
    UnauthorizedResolver,

    #[msg("No active dispute to resolve.")]
    NoActiveDispute,

    #[msg("Dispute reason cannot be empty.")]
    EmptyDisputeReason,
}





