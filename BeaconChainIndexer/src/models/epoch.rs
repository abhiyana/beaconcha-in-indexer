#[derive(Debug)]
pub struct Epoch {
    pub validator_set_size: u64,
    pub missed_attestations: u64,
    pub epoch_count: u64,
}