// src/bin/client.rs

use std::time::Duration;

pub mod api {
    tonic::include_proto!("api");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("TEST_SERVICE_ADDR").unwrap_or("http://127.0.0.1".to_string());
    let addr = format!("{addr}:50051");
    let mut client = api::test_service_client::TestServiceClient::connect(addr).await?;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("Shutting down client.");
                break;
            }
            _ = tokio::time::sleep(Duration::from_millis(500)) => {
                let request = tonic::Request::new(api::PingRequest {
                    message: "Hello from client".into(),
                });

                let response = client.ping(request).await?;
                println!("Client received PingResponse: {:?}", response.into_inner().reply);
            }
        }
    }

    Ok(())
}
