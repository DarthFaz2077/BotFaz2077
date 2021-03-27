use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub user_id: i64,
    pub level: i32,
    pub total_xp: i32,
    pub current_xp: i32,
}
