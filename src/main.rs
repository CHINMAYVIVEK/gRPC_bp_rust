

// fn main() {
//     println!("Hello World");
// }


use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Call the server's main function
    server::main().await?;
    Ok(())
}


