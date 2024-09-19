use crate::cart::checkout_server::Checkout;
use crate::cart::{CheckoutRequest, CheckoutResponse};
use tonic::{Request, Response, Status};

pub struct MyCheckoutService;

#[tonic::async_trait]
impl Checkout for MyCheckoutService {
    async fn checkout(
        &self,
        request: Request<CheckoutRequest>,
    ) -> Result<Response<CheckoutResponse>, Status> {
        let request = request.into_inner();
        // Implement your logic here
        Ok(Response::new(CheckoutResponse {
            confirmation: format!("Order {} processed.", request.order_id),
        }))
    }
}
