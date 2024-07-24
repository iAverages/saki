use axum::{routing::get, Router};
use protos::gen::helloworld::greeter_client::GreeterClient;
use protos::gen::helloworld::{HelloRequest, ImageResponse};

async fn root() -> String {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

    let request = tonic::Request::new(HelloRequest {
        name: "penis".into(),
    });

    let response = client.greet(request).await.unwrap();
    response.into_inner().response.to_string()
}

// async fn image() -> impl FStream<Item = ImageResponse>{
//     let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();
//
//     let request = tonic::Request::new(HelloRequest {
//         name: "penis".into(),
//     });
//
//     let stream = client.image(request).await.unwrap().into_inner();
//
//     // let mut stream = stream.take(5);
//
//     stream
//     //
//     // while let Some(item) = stream.next().await {
//     //     println!("\trecieved: {}", item.unwrap().image_data)
//     // }
//     //
//     // "lmfao".to_string()
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(root));
        // .route("/image", get(image));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
