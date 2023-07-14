# Beacon Chain Indexer

The Beacon Chain Indexer calculates the participation rate of the entire network based on the last five epochs. It utilizes a Rust and PostgreSQL database for data storage and retrieval.

## Installation

To run the Beacon Chain Indexer, make sure you have the following software installed:

- Rust (version 1.72.0-nightly or later)
- PostgreSQL (version 15.3 or later)

Please ensure that Rust and Cargo are properly set up on your system, and install PostgreSQL according to the instructions provided for your operating system.

## Database Setup

1. Create a new PostgreSQL database for the Beacon Chain Indexer.

2. Connect to the PostgreSQL database using your preferred method (e.g., psql command-line tool, GUI client).

3. Run the following SQL query to create the necessary table in the database:
   
```sql
CREATE TABLE IF NOT EXISTS slots (
    id                 BIGINT PRIMARY KEY,
    slot_number        BIGINT NOT NULL,
    validator_set_size BIGINT NOT NULL,
    epoch              BIGINT NOT NULL,
    missed_attestation BIGINT NOT NULL,
    created_at         TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```
This query creates a table named `slots` with columns for `id`, `slot_number`, `validator_set_size`, `epoch`, `missed_attestation`, and `created_at`.

## Getting Started

1. Clone this repository to your local machine.

2. Navigate to the project directory.

3. Update the database configuration in the `main.rs` file. Open the `main.rs` file and locate the `PostgresDatabase::new` function call. Modify the function arguments with your database connection details (host, port, user, password, dbname).

4. Build and run the project using Cargo.

5. The Beacon Chain Indexer will start processing slots. You can access the calculated participation rate by making a GET request to the `/network/participation_rate` endpoint.

## Project Structure

The project follows the following directory structure:

- `api/`: Contains the API-related code and handlers.
- `database/`: Contains the code for interacting with the PostgreSQL database.
- `indexer/`: Contains the code for processing slots and calculating the participation rate.
- `models/`: Contains the data models used in the project.
-  main/ : Contains the code for running HttpServer and processing slot scheduler

## Dependencies

The Beacon Chain Indexer project relies on the following external dependencies:

- `actix-web`: A powerful web framework for building APIs with Rust.
- `tokio-postgres`: A PostgreSQL client library for asynchronous Rust.
- `serde`: A library for serializing and deserializing Rust data structures.
- `reqwest`: A simple HTTP client for making API requests.

For a complete list of dependencies and their versions, refer to the `Cargo.toml` file.

## Future Enhancements

The Beacon Chain Indexer project provides functionality for calculating the participation rate of the entire network based on the last five epochs. However, there are several potential areas for future enhancement and additional functionality, It can be extend to calculate the Individual Validator Participation Rate.

## Contributing

Contributions to the Beacon Chain Indexer are welcome! If you find any issues or have suggestions for improvements.





