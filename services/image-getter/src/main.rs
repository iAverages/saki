use protos::r#gen::helloworld::ImageRequest;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};

use protos::gen::helloworld::greeter_server::{Greeter, GreeterServer};
use protos::gen::helloworld::{HelloRequest, HelloResponse, ImageResponse};

use std::fs::{self, File};
use std::io::Read;
use std::pin::Pin;
use std::result::Result;

#[derive(Debug, Default)]
pub struct MyGreeter {}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<ImageResponse, Status>> + Send>>;

const FILE_BUFFER_SIZE: usize = 8192;

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

    async fn image(
        &self,
        request: Request<ImageRequest>,
    ) -> Result<tonic::Response<ImageResponse>, Status> {
        let file = fs::read(request.into_inner().url)?;

        Ok(Response::new(ImageResponse { image_data: file }))
    }

    type ImageStreamStream = ResponseStream;

    async fn image_stream(
        &self,
        request: Request<ImageRequest>,
    ) -> Result<Response<Self::ImageStreamStream>, Status> {
        let mut f = File::open(request.into_inner().url)?;
        let file_size = f.metadata().unwrap().len();

        let repeat = std::iter::repeat_with(move || {
            let mut buffer = [0; FILE_BUFFER_SIZE];

            let _ = f.read(&mut buffer).expect("Failed to read buffer");

            ImageResponse {
                image_data: buffer.to_vec(),
            }
        })
        .take((file_size as usize / FILE_BUFFER_SIZE) + 1);

        let mut stream = Box::pin(tokio_stream::iter(repeat));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(124);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        println!("queued to send");
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ImageStreamStream
        ))
        // todo:
        // stream file from disk
        // create stream that can be used by grpc
        // stonk
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
