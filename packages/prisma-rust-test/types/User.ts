#[derive(sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub name: String,
}