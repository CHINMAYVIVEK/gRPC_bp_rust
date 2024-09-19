use std::error::Error;
use tokio_postgres::{NoTls, Client, Config};

pub async fn connect_db(db_config: &DatabaseConfig) -> Result<Client, Box<dyn Error>> {
    let config = Config::new()
        .user(&db_config.user)
        .password(&db_config.password)
        .host(&db_config.host)
        .port(db_config.port)
        .dbname(&db_config.dbname);
    
    let (client, connection) = config.connect(NoTls).await?;
    
    // Spawn a new task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {:?}", e);
        }
    });

    Ok(client)
}
