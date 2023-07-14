pub mod slot;
pub mod epoch;
pub mod AttestationData;
pub mod AttestationResponse;
pub mod ResultData;

// Re-export the data models for convenient access
pub use slot::Slot;
pub use epoch::Epoch;
pub use AttestationData::AttData;
