use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::{routing::get, Router};
use protos::gen::helloworld::greeter_client::GreeterClient;
use protos::gen::helloworld::HelloRequest;
use protos::r#gen::database::database_client::DatabaseClient;
use protos::r#gen::database::GetFileMetaRequest;
use protos::r#gen::helloworld::ImageRequest;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tonic::Request;

async fn root() -> String {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

    let request = tonic::Request::new(HelloRequest {
        name: "penis".into(),
    });

    let response = client.greet(request).await.unwrap();
    response.into_inner().response
}

async fn image_stream_video() -> Result<impl IntoResponse, ()> {
    let path = "/home/dan/Downloads/urzHKjY.mp4";
    image_stream_impl(path).await
}

async fn image_stream_small() -> Result<impl IntoResponse, ()> {
    let path = "/home/dan/Pictures/Screenshot_20240726_224035.png";
    image_stream_impl(path).await
}

async fn image_stream_big() -> Result<impl IntoResponse, ()> {
    let path = "/home/dan/wallpapers/shoshimin-06.png";
    image_stream_impl(path).await
}

async fn image_stream_impl(path: &str) -> Result<impl IntoResponse, ()> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();
    let request = tonic::Request::new(ImageRequest {
        url: path.to_string(),
    });

    let mut image = client.image_stream(request).await.unwrap().into_inner();

    let (tx, rx) = mpsc::channel::<Result<Vec<u8>, Infallible>>(2);

    tokio::spawn(async move {
        while let Some(item) = image.next().await {
            let mut chunked = vec![];
            let part = item.unwrap().image_data;
            for byte in part {
                chunked.push(byte);
                if chunked.len() > 1024 {
                    tx.send(Ok(chunked.clone())).await.unwrap();
                    chunked.clear();
                }
            }
            tx.send(Ok(chunked.clone())).await.unwrap();
            chunked.clear();
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let body = Body::from_stream(stream);
    let mut response = Response::builder().status(StatusCode::OK);

    if path.ends_with(".mp4") {
        response = response.header("Content-Type", "video/mp4");
    }

    Ok(response.body(body).unwrap())
}

#[derive(Serialize, Deserialize)]
struct ImageMeta {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub upload_date: String,
}

async fn image_meta(Path(key): Path<String>) -> Result<impl IntoResponse, ()> {
    let mut client = DatabaseClient::connect("http://[::1]:50052").await.unwrap();
    let request = Request::new(GetFileMetaRequest { key });

    match client.get_file_meta(request).await {
        Ok(response) => {
            let meta = response.into_inner();
            Ok(Json(ImageMeta {
                id: meta.id,
                name: meta.name,
                size: meta.size,
                upload_date: meta.upload_date,
            }))
        }
        // TODO: Return proper error here
        Err(_) => Err(()),
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(root))
        .route("/image/video", get(image_stream_video))
        .route("/image/big", get(image_stream_big))
        .route("/image/meta/:key", get(image_meta))
        .route("/image/small", get(image_stream_small));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
