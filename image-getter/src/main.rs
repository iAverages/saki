use protos::r#gen::helloworld::ImageRequest;
use std::fs;
use std::pin::Pin;
use tokio_stream::Stream;
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
            Ok(()) => return Some(Ok(buffer)),
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
