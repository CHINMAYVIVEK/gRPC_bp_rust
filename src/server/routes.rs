use crate::app::health::welcome::HealthCheckService;
use crate::app::health::healthcheck::health_server::Health;
use crate::app::health::healthcheck::HealthCheckRequest;
use serde::Deserialize;
use std::fs;
use tonic::{Request, Response, Status};
use std::any::Any;

#[derive(Deserialize)]
struct Route {
    path: String,
    method: String,
    handler: String,
}

#[derive(Deserialize)]
struct EndpointConfig {
    routes: Vec<Route>,
}

/// Load routes from a YAML configuration file
pub fn load_routes() -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    let config_path = "src/server/endpoints.yaml";
    let config_str = fs::read_to_string(config_path)?;
    let config: EndpointConfig = serde_yaml::from_str(&config_str)?;
    Ok(config.routes)
}

/// Handle incoming requests based on the route configuration
pub async fn handle_request(
    route: &Route,
    _request: Request<()>, // The original request, ignored for the health check
) -> Result<Response<Box<dyn Any + Send>>, Status> {
    match route.handler.as_str() {
        "healthcheck" => {
            // let health_service = HealthCheckService::default();

            // // Create a dummy HealthCheckRequest
            // let health_check_request = HealthCheckRequest {
            //     // Add fields if necessary
            // };

            // // Create a new tonic request with the health check request
            // let tonic_request = Request::new(health_check_request);

            let health_service = HealthCheckService::default();
            let health_check_request = HealthCheckRequest::default(); // Create a default request
            let tonic_request = Request::new(health_check_request);
            
            // Call the health check service
            let response = health_service.check(tonic_request).await?;

            // Return the response wrapped in a Box
            Ok(Response::new(Box::new(response)))
        }
        // Add more handlers for different services here
        _ => Err(Status::not_found("Handler not found")),
    }
}
