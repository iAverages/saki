#[derive(sqlx::FromRow)]
pub struct DBFilemeta {
    pub id: String,
    pub name: String,
    pub size: i32,
    pub upload_date: sqlx::types::chrono::NaiveDateTime,
}

