use tonic::transport::Server;
use std::error::Error;
use crate::config; 

mod lib; // Service registration module
mod routes; // Module for handling routes

pub async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse()?;
    let mut server = Server::builder();

    // Load endpoints configuration
    let endpoints_config = config::load_config("src/server/endpoints.yaml")?;
    
    // Register services based on the configuration
    lib::register_services(&mut server, &endpoints_config)?;

    // Load and register routes if necessary
    routes::load_routes()?;

    // Logic to attach routes to the server would go here

    println!("Starting server on {}", addr);
    // Start the server
    server.serve(addr).await?;
    Ok(())
}
