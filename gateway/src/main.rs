use axum::http::header;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use bytes::Bytes;
use protos::gen::helloworld::greeter_client::GreeterClient;
use protos::gen::helloworld::HelloRequest;
use protos::r#gen::helloworld::ImageRequest;

async fn root() -> String {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

    let request = tonic::Request::new(HelloRequest {
        name: "penis".into(),
    });

    let response = client.greet(request).await.unwrap();
    response.into_inner().response
}

async fn image() -> Result<impl IntoResponse, ()> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

    let path = "/home/dan/Pictures/Screenshot_20240603_000557.png";

    let request = tonic::Request::new(ImageRequest {
        url: path.to_string()
    });

    let image = client.image(request).await.unwrap() .into_inner();
    let bytes = Bytes::from(image.image_data);

    let headers = [(header::CONTENT_TYPE, "image/png")];

    Ok((headers,bytes).into_response())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(root))
        .route("/image", get(image));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
