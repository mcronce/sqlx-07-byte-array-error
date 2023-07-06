use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct UserRow {
	id: u16,
	password_hash: Vec<u8>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let db_url = std::env::var("DATABASE_URL").unwrap();
	dbg!(&db_url);
	let pool = sqlx::MySqlPool::connect(&db_url).await.unwrap();
	let mut tx = pool.begin().await.unwrap();
	let users = sqlx::query_as!(UserRow, "SELECT * FROM users_test").fetch_all(&mut *tx).await.unwrap();
	dbg!(users);
}

