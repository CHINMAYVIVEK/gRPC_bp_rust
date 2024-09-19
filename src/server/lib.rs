use serde_yaml::Value;
use tonic::transport::Server;
use crate::app::health::{welcome, healthcheck}; 

// Function to register services based on the configuration
pub fn register_services(
    server: &mut Server, 
    endpoints_config: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the health check service is enabled in the configuration
    if let Some(health_check_enabled) =
        endpoints_config["services"]["health_check"]["enabled"].as_bool()
    {
        if health_check_enabled {
            // Create the health check service instance
            let health_service = welcome::HealthCheckService::default(); // Use the correct service
            
            // Register the health check service with the server
            server.add_service(
                healthcheck::health_server::HealthServer::new(health_service),
            );
        }
    }

    // Register other services based on the configuration here
    // Extend this section to include more services as needed

    Ok(())
}
