use crate::cart::checkout::MyCheckoutService;
use crate::homepage::banner::MyBannerService;
use crate::product::get_product::MyGetProductService;
use crate::proto::cart::checkout_server::CheckoutServer;
use crate::proto::homepage::banner_server::BannerServer;
use crate::proto::product::get_product_server::GetProductServer;
use crate::proto::user::user_service_server::UserServiceServer;
use crate::user::login::MyUserService;
use serde_yaml::Value;
use std::fs::File;
use std::io::Read;
use tonic::transport::Server;

mod lib;
mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the server config
    let mut config_file = File::open("server/config.yaml")?;
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str)?;
    let config: Value = serde_yaml::from_str(&config_str)?;

    let addr = config["server"]["address"].as_str().unwrap().parse()?;

    // Load the endpoints config
    let mut endpoints_file = File::open("server/endpoints.yaml")?;
    let mut endpoints_str = String::new();
    endpoints_file.read_to_string(&mut endpoints_str)?;
    let endpoints_config: Value = serde_yaml::from_str(&endpoints_str)?;

    let mut server = Server::builder();

    // Add services based on endpoints config
    if endpoints_config["services"]["user_service"]["enabled"]
        .as_bool()
        .unwrap()
    {
        server = server.add_service(UserServiceServer::new(MyUserService));
    }
    if endpoints_config["services"]["product_service"]["enabled"]
        .as_bool()
        .unwrap()
    {
        server = server.add_service(GetProductServer::new(MyGetProductService));
    }
    if endpoints_config["services"]["cart_service"]["enabled"]
        .as_bool()
        .unwrap()
    {
        server = server.add_service(CheckoutServer::new(MyCheckoutService));
    }
    if endpoints_config["services"]["homepage_service"]["enabled"]
        .as_bool()
        .unwrap()
    {
        server = server.add_service(BannerServer::new(MyBannerService));
    }

    println!("Starting server on {}", addr);
    server.serve(addr).await?;

    Ok(())
}
