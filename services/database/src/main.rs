mod config;
mod types;

use crate::types::filemeta::DBFilemeta;
use dotenv::dotenv;
use protos::r#gen::database::database_server::{Database, DatabaseServer};
use protos::r#gen::database::{GetFileMetaRequest, GetFileMetaResponse};
use sqlx::mysql::MySqlPoolOptions;
use std::result::Result;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};

use self::config::load_config;

pub struct ContextOptions {
    pub db_url: String,
}

pub struct Context {
    pub db: sqlx::MySqlPool,
}

impl Context {
    pub async fn new(options: &ContextOptions) -> Self {
        let db_pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&options.db_url)
            .await
            .expect("Failed to connect to database");

        Context { db: db_pool }
    }
}

pub struct DatabaseService {
    pub ctx: Context,
}

#[tonic::async_trait]
impl Database for DatabaseService {
    async fn get_file_meta(
        &self,
        request: Request<GetFileMetaRequest>,
    ) -> Result<Response<GetFileMetaResponse>, Status> {
        let data = request.into_inner();

        match sqlx::query_as::<_, DBFilemeta>("SELECT * FROM FileMeta WHERE id = ?")
            .bind(data.key)
            .fetch_one(&self.ctx.db)
            .await
        {
            Ok(query) => {
                return Ok(Response::new(GetFileMetaResponse {
                    id: query.id,
                    name: query.name,
                    size: query.size as u64,
                    upload_date: query.upload_date.to_string(),
                }))
            }
            Err(_) => return Err(tonic::Status::new(Code::NotFound, "file not found")),
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let config = load_config().await;
    let addr = "[::1]:50052".parse().unwrap();
    let options = ContextOptions {
        db_url: config.database_url,
    };

    let context = Context::new(&options).await;

    let database_service = DatabaseService { ctx: context };

    Server::builder()
        .add_service(DatabaseServer::new(database_service))
        .serve(addr)
        .await?;

    Ok(())
}
