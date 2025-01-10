use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};

use crate::configs::db_config::DatabaseConfig;

/// function to establish a connection to the MongoDB server
pub async fn establish_connection() -> Database {
    // Replace the placeholder with your Atlas connection string
    let db_config: DatabaseConfig = DatabaseConfig::default();

    let mut client_options: ClientOptions = ClientOptions::parse(db_config.url)
        .await
        .expect("Failed to parse options.");

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api: ServerApi = ServerApi::builder().version(ServerApiVersion::V1).build();

    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client: Client =
        Client::with_options(client_options).expect("Failed to initialize client.");

    // Access the database
    let database: Database = client.database(db_config.name.as_str());

    log::info!("You successfully connected to MongoDB!");
    database
}
