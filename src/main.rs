mod service;
mod web;

use crate::{service::pg::PostgresStepCounterService, web::web::serve};

#[tokio::main]
async fn main() {
    // Prepare service & storage
    let svc = PostgresStepCounterService::new().await.unwrap();
    svc.init().await.unwrap();

    // Open the service up to the web
    serve(svc).await.unwrap();
}
