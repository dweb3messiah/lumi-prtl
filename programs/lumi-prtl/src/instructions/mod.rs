pub mod init_shipment;
pub use init_shipment::*;
// Compare this snippet from programs/lumi-prtl/src/instructions/mod.rs:

pub mod buy;
pub use buy::*;

pub mod update_shipment;
pub use update_shipment::*;

pub mod pay_for_shipment;
pub use pay_for_shipment::*;
