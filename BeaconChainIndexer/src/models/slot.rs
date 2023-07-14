#[derive(Debug)]
pub struct Slot {
    pub slot_number: i64,
    pub missed_attestations: i64,
    pub validator_set_size: i64,
    pub epoch: i64,
}
