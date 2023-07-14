use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AttData {
    pub aggregationbits: String,
    pub validators: Vec<u64>,
    pub target_epoch: u64,
}