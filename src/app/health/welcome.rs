use tonic::{Request, Response, Status};
use crate::app::health::healthcheck::{health_server::Health,HealthCheckRequest, HealthCheckResponse,health_check_response::ServingStatus};

#[derive(Default)]
pub struct HealthCheckService;

#[tonic::async_trait]
impl Health for HealthCheckService {
    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let response = HealthCheckResponse {
            status: ServingStatus::Serving as i32, 
        };
        Ok(Response::new(response))
    }
}
