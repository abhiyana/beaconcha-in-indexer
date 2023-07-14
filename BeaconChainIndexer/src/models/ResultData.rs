use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResultData {
    pub missed_attestations: u64,
    pub validator_set_size: u64,
}
