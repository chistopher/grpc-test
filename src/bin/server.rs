use crate::api::test_service_server::TestServiceServer;
use rand::Rng; // Add this import
use tokio::signal;
use tonic::{transport::Server, Request, Response, Status};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Default)]
pub struct TestService {
    pub server_id: String,
}

#[tonic::async_trait]
impl api::test_service_server::TestService for TestService {
    async fn ping(
        &self,
        request: Request<api::PingRequest>,
    ) -> Result<Response<api::PingResponse>, Status> {
        let message = request.into_inner().message;
        println!(
            "Server {} received PingRequest: {message:?}",
            self.server_id
        );

        let reply = api::PingResponse {
            reply: format!("Hello from server: {}", self.server_id),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?; // Changed from "[::1]:50051" to "127.0.0.1:50051"
    let server_id: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect(); // Generate random server_id

    println!("Server {} listening on {}", &server_id, addr);
    let test_service = TestService { server_id };
    let test_server = TestServiceServer::new(test_service);
    Server::builder()
        .add_service(test_server)
        .serve_with_shutdown(addr, async {
            signal::ctrl_c().await.unwrap();
            println!("Shutting down server.");
        })
        .await?;

    Ok(())
}
