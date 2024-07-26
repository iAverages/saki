use protos::r#gen::helloworld::ImageRequest;
use std::fs::{self, File};
use std::pin::Pin;
use std::result::Result;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};

use protos::gen::helloworld::greeter_server::{Greeter, GreeterServer};
use protos::gen::helloworld::{HelloRequest, HelloResponse, ImageResponse};
use std::{error::Error, io::Read};

#[derive(Debug, Default)]
pub struct MyGreeter {}

type EchoResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<ImageResponse, Status>> + Send>>;

const BUFFER_SIZE: usize = 5;

fn read_file<R: Read>(mut reader: R) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0_u8; BUFFER_SIZE];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        let string_slice = std::str::from_utf8(&buffer[..count])?;

        // Read BUFFER_SIZE bytes in each loop
        // Change print! to println!, to see the result.

        print!("{string_slice}");
    }

    Ok(())
}

use std::io::{self, ErrorKind};

pub struct ToChunks<R> {
    reader: R,
    chunk_size: usize,
}

impl<R: Read> Iterator for ToChunks<R> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0u8; self.chunk_size];
        match self.reader.read_exact(&mut buffer) {
            Ok(()) => Some(Ok(buffer)),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub trait IterChunks {
    type Output;

    fn iter_chunks(self, len: usize) -> Self::Output;
}

impl<R: Read> IterChunks for R {
    type Output = ToChunks<R>;

    fn iter_chunks(self, len: usize) -> Self::Output {
        ToChunks {
            reader: self,
            chunk_size: len,
        }
    }
}

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
