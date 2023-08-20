use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/database_name")
        .await?;

    let rows_affected = sqlx::query("INSERT INTO users (name) VALUES ('Bob')")
        .execute(&pool)
        .await?;

    let rec: (i64, String) = sqlx::query_as("SELECT id, name FROM users WHERE id = ?")
        .bind(5)
        .fetch_one(&pool)
        .await?;

    println!("rows_affected = {:?}", rows_affected);
    println!("rec = {:?}", rec);

    Ok(())
}
