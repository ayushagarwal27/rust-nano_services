
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;


pub async fn run_migrations() {
    println!("Migrating users database...");
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.ignore_missing = true;
    let result = migrations.run(&*SQLX_POSTGRES_POOL)
                           .await.unwrap();
    println!(
        "users database migrations completed: {:?}", 
        result
    );
}
