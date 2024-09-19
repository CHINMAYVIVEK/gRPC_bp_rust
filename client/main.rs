use proto::user::user_service_client::UserServiceClient;
use proto::user::LoginRequest;
use serde_yaml::Value;
use std::fs::File;
use std::io::Read;
use tonic::transport::Channel;

mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the client config
    let mut config_file = File::open("client/config.yaml")?;
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str)?;
    let config: Value = serde_yaml::from_str(&config_str)?;

    let addr = config["server"]["address"].as_str().unwrap();

    let channel = Channel::from_static(addr).connect().await?;
    let mut client = UserServiceClient::new(channel);

    let request = tonic::Request::new(LoginRequest {
        username: "admin".into(),
        password: "password".into(),
    });

    let response = client.login(request).await?;

    println!("Response from server: {:?}", response);

    Ok(())
}
