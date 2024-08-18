mod config;
mod types;

use dotenv::dotenv;
use protos::r#gen::database::database_server::{Database, DatabaseServer};
use protos::r#gen::database::{GetFileMetaRequest, GetFileMetaResponse};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::query;
use std::result::Result;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

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

#[derive(sqlx::FromRow)]
pub struct DBFileMeta {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub upload_date: String,
}

#[tonic::async_trait]
impl Database for DatabaseService {
    async fn get_file_meta(
        &self,
        request: Request<GetFileMetaRequest>,
    ) -> Result<Response<GetFileMetaResponse>, Status> {
        let data = request.into_inner();

        let query = sqlx::query_as::<_, DBFileMeta>("SELECT * FROM FileMeta WHERE id = ?")
            .bind(data.key)
            .fetch_one(&self.ctx.db)
            .await
            .unwrap();

        let response = GetFileMetaResponse {
            id: query.id,
            name: query.name,
            size: query.size,
            upload_date: query.upload_date,
        };

        Ok(Response::new(response))
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
