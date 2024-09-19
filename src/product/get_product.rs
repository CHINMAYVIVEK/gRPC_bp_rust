use tonic::{Request, Response, Status};
use crate::product::{GetProductRequest, GetProductResponse};
use crate::product::get_product_server::GetProduct;

pub struct MyGetProductService;

#[tonic::async_trait]
impl GetProduct for MyGetProductService {
    async fn get_product(&self, request: Request<GetProductRequest>) -> Result<Response<GetProductResponse>, Status> {
        let request = request.into_inner();
        // Implement your logic here
        Ok(Response::new(GetProductResponse {
            product_name: format!("Product ID {} details.", request.product_id),
        }))
    }
}
