#![deny(clippy::integer_arithmetic)]
#![deny(clippy::disallowed_method)]

#[macro_use]
mod macros;

pub mod common;
pub mod genesis;
pub mod per_block_processing;
pub mod per_epoch_processing;
pub mod per_slot_processing;
pub mod state_advance;
// FIXME(altair): re-enable
// pub mod test_utils;
pub mod verify_operation;

pub use genesis::{
    eth2_genesis_time, initialize_beacon_state_from_eth1, is_valid_genesis_state,
    process_activations,
};
pub use per_block_processing::{
    block_signature_verifier, errors::BlockProcessingError, per_block_processing, signature_sets,
    BlockSignatureStrategy, BlockSignatureVerifier, VerifySignatures,
};
// FIXME(altair): consider process_epoch name
pub use per_epoch_processing::{
    errors::EpochProcessingError, process_epoch as per_epoch_processing,
};
pub use per_slot_processing::{per_slot_processing, Error as SlotProcessingError};
pub use verify_operation::{SigVerifiedOp, VerifyOperation};
