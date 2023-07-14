mod api;
mod database;
mod indexer;
mod models;

use database::postgres::PostgresDatabase;
use indexer::Indexer;
use crate::api::handlers;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use actix_web::middleware::Logger;
use env_logger;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Create a new instance of the PostgresDatabase
    let database = Arc::new(Mutex::new(
        PostgresDatabase::new("localhost", 5432, "abhishek", "Sakoli@1998", "mydatabase")
            .await
            .expect("Failed to create PostgresDatabase instance"),
    ));

    // Create an instance of the Indexer and pass the PostgresDatabase to it
    let indexer = Arc::new(Indexer::new(database.clone()));

    // Spawn a separate task to run the process_slots method periodically
    tokio::task::spawn_blocking(move || {
        actix_rt::System::new().block_on(async move {
            loop {
                // Acquire the lock to access the database
                if let Ok(mut database) = database.try_lock() {
                    Indexer::process_slots(&mut database).await.unwrap_or_else(|err| {
                        eprintln!("Error in process_slot: {}", err);
                    });
                }

                // Sleep for the desired interval (Average time of adding new slot is 12 sec)
                sleep(Duration::from_secs(10)).await;
            }
        })
    });

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(indexer.clone()))
            .route("/network/participation_rate", web::get().to(handlers::get_network_participation_rate))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
