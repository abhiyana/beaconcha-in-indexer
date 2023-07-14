pub use crate::models::AttestationData::AttData;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AttestationResponse {
    pub data: Vec<AttData>,
}
