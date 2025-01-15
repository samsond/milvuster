use tonic::transport::Endpoint;

use crate::proto::milvus::CreateDatabaseRequest;
use crate::proto::milvus::milvus_service_client::MilvusServiceClient;
use crate::proto::milvus::ListDatabasesRequest;
use tonic::Request;


pub async fn connect_to_milvus(endpoint: &str) -> Result<MilvusServiceClient<tonic::transport::Channel>, Box<dyn std::error::Error>> {

    // Create an Endpoint to connect to Milvus
    let endpoint = Endpoint::from_shared(endpoint.to_string())?;

    // Create a client for Milvus using connect_custom
    let client = MilvusServiceClient::connect_custom(endpoint).await?;

    Ok(client)
}

pub async fn create_database(client: &mut MilvusServiceClient<tonic::transport::Channel>, db_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let request = CreateDatabaseRequest {
        base: None,
        db_name: db_name.to_string(),
        properties: vec![], 
    };

    let response = client.create_database(Request::new(request)).await?;
    println!("Database created successfully: {:?}", response.into_inner());

    Ok(())
}

pub async fn list_databases(client: &mut MilvusServiceClient<tonic::transport::Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let request = ListDatabasesRequest {
        base: None,

    };

    let response = client.list_databases(Request::new(request)).await?;
    let db_names = response.into_inner().db_names;
    println!("Databases: {:?}", db_names);

    Ok(())
}