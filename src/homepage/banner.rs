use crate::homepage::banner_server::Banner;
use crate::homepage::{BannerRequest, BannerResponse};
use tonic::{Request, Response, Status};

pub struct MyBannerService;

#[tonic::async_trait]
impl Banner for MyBannerService {
    async fn get_banner(
        &self,
        request: Request<BannerRequest>,
    ) -> Result<Response<BannerResponse>, Status> {
        let request = request.into_inner();
        // Implement your logic here
        Ok(Response::new(BannerResponse {
            message: format!("Banner for {} is active.", request.page),
        }))
    }
}
