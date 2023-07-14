use actix_web::{web, Responder};
use std::sync::Arc;
use crate::indexer::Indexer;

/// Handler for the "get_network_participation_rate" endpoint.
pub async fn get_network_participation_rate(indexer: web::Data<Arc<Indexer>>) -> impl Responder {
    let participation_rate = indexer.calculate_network_participation_rate().await
        .map_err(|err| {
            eprintln!("Error calculating participation rate: {}", err);
            err
        })
        .unwrap_or_default();

    format!("Participation Rate: {:.2}%", participation_rate * 100.0)
}
