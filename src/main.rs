use milvuster::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "http://127.0.0.1:19530"; // Replace with the actual endpoint in the real world
    // Call the connect_to_milvus function and handle errors
    let mut client = match client::connect_to_milvus(&endpoint).await {
        Ok(client) => {
            println!("Connected successfully to Milvus.");
            client
        }
        Err(e) => {
            eprintln!("Error during connect: {:?}", e);  // Print detailed error
            return Err(e);  
        }
    };

    // Create a new database
    match client::create_database(&mut client, "test_db").await {
        Ok(_) => {
            println!("Database created successfully.");
        }
        Err(e) => {
            eprintln!("Error during create database: {:?}", e);  // Print detailed error
            return Err(e);  
        }
    }

    // List all databases
    match client::list_databases(&mut client).await {
        Ok(_) => {
            println!("Listed databases successfully.");
        }
        Err(e) => {
            eprintln!("Error during list databases: {:?}", e);  // Print detailed error
            return Err(e);  
        }
    }

    Ok(())
}
