#[derive(sqlx::FromRow)]
pub struct DBFilemeta {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub upload_date: TODO:,
}