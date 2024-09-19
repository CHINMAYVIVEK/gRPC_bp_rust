use crate::health::health_check_server::HealthCheck;
use crate::health::{HealthCheckRequest, HealthCheckResponse};
use tonic::{Request, Response, Status};

pub struct MyHealthCheckService;

#[tonic::async_trait]
impl HealthCheck for MyHealthCheckService {
    async fn check(
        &self,
        _: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            status: "Healthy".into(),
        }))
    }
}
