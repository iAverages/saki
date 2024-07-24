use std::pin::Pin;
use tokio_stream::{Stream};
use tonic::{transport::Server, Request, Response, Status};

use protos::gen::helloworld::greeter_server::{Greeter, GreeterServer};
use protos::gen::helloworld::{HelloRequest, HelloResponse, ImageResponse};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn greet(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloResponse {
            response: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    type ImageStream = Pin<Box<dyn Stream<Item = Result<ImageResponse, Status>> + Send>>;

    async fn image(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::ImageStream>, Status> {
        todo!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
