use crate::database::PostgresDatabase;
use crate::models::AttestationResponse::AttestationResponse;
use crate::models::Slot;
use num_bigint::BigUint;
use num_traits::Num;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

pub struct Indexer {
    database: Arc<Mutex<PostgresDatabase>>,
}

impl Indexer {
    pub fn new(database: Arc<Mutex<PostgresDatabase>>) -> Indexer {
        // Create a new instance of Indexer with the provided database
        Indexer { database }
    }

    pub async fn process_slots(db: Arc<Mutex<PostgresDatabase>>) -> Result<(), Box<dyn Error>> {
        let mut database = db.lock().unwrap(); // Acquire the lock to access the database

        // Fetch the latest slot number from the database
        let latest_slot: i64 = database.get_latest_slot().await?;

        // Build the URL for the latest slot data
        let latest_slot_url = format!("https://beaconcha.in/api/v1/slot/latest");

        // Make the API call to get the latest slot data
        let latest_slot_data = Self::make_api_call(&latest_slot_url).await?;
        let latest_slot_number: i64 = latest_slot_data["data"]["slot"].as_i64().unwrap();

        // Check if there are new slots available
        if latest_slot_number > latest_slot {
            // Determine the starting slot number based on the existing slots in the database
            let start_slot = if latest_slot == 0 {
                latest_slot_number
            } else {
                latest_slot + 1
            };

            // Process slots from the starting slot to the latest slot obtained from the API
            for slot_number in start_slot..=latest_slot_number {
                let slot_url = format!(
                    "https://beaconcha.in/api/v1/slot/{}/attestations",
                    slot_number
                );

                // Make the API call to get the slot data
                match Self::make_api_call(&slot_url).await {
                    Ok(json_data) => {
                        let attestation_response: AttestationResponse =
                            serde_json::from_value(json_data)?;

                        // Access the data field of the response struct
                        let data = attestation_response.data;

                        // Initialize variables to store aggregated data
                        let mut missed_attestations: i64 = 0;
                        let mut validator_set_size: i64 = 0;
                        let mut epoch: i64 = 0;

                        // Process each attestation of committee in the data
                        for attestation in data {
                            let committee_size = attestation.validators.len() as i64;
                            missed_attestations +=
                                Self::calculate_missed_attestations(&attestation.aggregationbits)?;
                            validator_set_size += committee_size;
                            if attestation.target_epoch as i64 > epoch {
                                epoch = attestation.target_epoch as i64;
                            }
                        }

                        // Store the slot data in the database
                        let slot = Slot {
                            slot_number: slot_number as i64,
                            validator_set_size,
                            missed_attestations,
                            epoch,
                        };

                        database.store_slot(slot).await?;
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }

                // Sleep for a short duration to avoid rate limiting
                sleep(Duration::from_millis(100)).await;
            }
        }

        Ok(())
    }

    fn calculate_missed_attestations(
        aggregationbits: &str,
    ) -> Result<i64, num_bigint::ParseBigIntError> {
        // Calculate the number of missed attestations based on the aggregation bits
        let trimmed_bits = aggregationbits.trim_start_matches("0x");
        let big_int = BigUint::from_str_radix(trimmed_bits, 16)?;
        let binary = big_int.to_str_radix(2);
        let count_zeros = binary.chars().filter(|&c| c == '0').count() as i64;
        Ok(count_zeros)
    }

    pub async fn make_api_call(url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // Create a new reqwest client
        let client = reqwest::Client::new();

        // Send the GET request
        let response = client
            .get(url)
            .header("accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;

            let json_data: serde_json::Value = serde_json::from_str(&body)?;

            Ok(json_data)
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }

    pub async fn calculate_network_participation_rate(&self) -> Result<f64, Box<dyn Error>> {
        let database = self.database.lock().unwrap(); 

        // Retrieve the necessary data for the last five epochs
        let epoch_data = database.get_last_five_epochs_data().await?;

        let total_validator_set_size = epoch_data.validator_set_size;
        let total_missed_attestations = epoch_data.missed_attestations;
        let epoch_count = epoch_data.epoch_count;

        // Calculate the participation rate
        let participation_rate = 1.0
            - (total_missed_attestations as f64
                / (epoch_count as f64 * total_validator_set_size as f64));

        Ok(participation_rate)
    }
}
