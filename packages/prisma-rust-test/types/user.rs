#[derive(sqlx::FromRow)]
pub struct DBUser {
    pub id: u64,
    pub email: String,
    pub name: String,
}