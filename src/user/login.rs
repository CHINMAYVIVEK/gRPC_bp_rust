use crate::user::user_service_server::UserService;
use crate::user::{LoginRequest, LoginResponse};
use tonic::{Request, Response, Status};

pub struct MyUserService;

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        if req.username == "admin" && req.password == "password" {
            Ok(Response::new(LoginResponse {
                success: true,
                message: "Login successful".into(),
            }))
        } else {
            Ok(Response::new(LoginResponse {
                success: false,
                message: "Invalid credentials".into(),
            }))
        }
    }
}
