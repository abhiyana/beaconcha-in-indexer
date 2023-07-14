use crate::models::Epoch;
use crate::models::Slot;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio_postgres::{Client, NoTls};

/// Database struct for Postgres database interactions.
pub struct PostgresDatabase {
    client: Arc<Mutex<Client>>,
}

impl PostgresDatabase {
    /// Creates a new instance of PostgresDatabase.
    /// Connects to the Postgres database using the provided connection details.
    pub async fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        dbname: &str,
    ) -> Result<PostgresDatabase, Box<dyn std::error::Error>> {
        let connection_string = format!(
            "host={} port={} user={} password={} dbname={}",
            host, port, user, password, dbname
        );

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

        // Spawn a task to run the connection
        let connection_task = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let client = Arc::new(Mutex::new(client));

        Ok(PostgresDatabase { client })
    }

    /// Stores a slot in the database.
    pub async fn store_slot(&self, slot: Slot) -> Result<(), Box<dyn Error>> {
        let client = self.client.lock().unwrap(); // Acquire the lock to access the client
        let stmt = client
            .prepare("INSERT INTO slots (slot_number, validator_set_size, epoch, missed_attestation) VALUES ($1, $2, $3, $4)")
            .await?;

        client
            .execute(
                &stmt,
                &[
                    &slot.slot_number,
                    &slot.validator_set_size,
                    &slot.epoch,
                    &slot.missed_attestations,
                ],
            )
            .await?;

        Ok(())
    }

    /// Retrieves the latest slot number from the database.
    pub async fn get_latest_slot(&self) -> Result<i64, Box<dyn Error>> {
        let client = self.client.lock().unwrap(); // Acquire the lock to access the client
        let stmt = client
            .prepare("SELECT slot_number FROM slots ORDER BY slot_number DESC LIMIT 1")
            .await?;
        let rows = client.query(&stmt, &[]).await?;

        if let Some(row) = rows.get(0) {
            let slot_number: i64 = row.try_get("slot_number")?;
            Ok(slot_number)
        } else {
            Ok(0) // Return 0 if there are no slots in the table
        }
    }

    /// Retrieves the data for the last five epochs from the database.
    pub async fn get_last_five_epochs_data(&self) -> Result<Epoch, Box<dyn Error>> {
        let client = self.client.lock().unwrap(); // Acquire the lock to access the client

        // Query the database to fetch the necessary data for the last five epochs or all available epochs if less than five
        let query = "SELECT (SUM(CAST(validator_set_size AS bigint))::numeric)::text as validator_set_size,
                     (SUM(CAST(missed_attestation AS bigint))::numeric)::text as missed_attestations,
                     (COUNT(DISTINCT epoch)::numeric)::text as epoch_count
                     FROM slots
                     WHERE epoch > (SELECT MAX(epoch) - 5 FROM slots)";
        let rows = client.query(query, &[]).await?;

        // Iterate over the rows and calculate the sum of validator set size, missed attestations, and epoch count
        let mut total_validator_set_size = 0;
        let mut total_missed_attestations = 0;
        let mut total_epoch_count = 0;

        for row in rows {
            let validator_set_size_str: String = row.get("validator_set_size");
            let missed_attestations_str: String = row.get("missed_attestations");
            let epoch_count_str: String = row.get("epoch_count");

            println!("A {} B {} C {}", validator_set_size_str, missed_attestations_str, epoch_count_str);

            let validator_set_size: u64 = validator_set_size_str.parse()?;
            let missed_attestations: u64 = missed_attestations_str.parse()?;
            let epoch_count: u64 = epoch_count_str.parse()?;

            println!("a {} b{} c{}", validator_set_size, missed_attestations, epoch_count);

            total_validator_set_size += validator_set_size;
            total_missed_attestations += missed_attestations;
            total_epoch_count += epoch_count;
        }

        // Create an Epoch struct with the calculated values
        let epoch_data = Epoch {
            validator_set_size: total_validator_set_size,
            missed_attestations: total_missed_attestations,
            epoch_count: total_epoch_count,
        };

        Ok(epoch_data)
    }
}
