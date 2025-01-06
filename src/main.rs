#[macro_use]
extern crate rocket;

mod routes;
mod models;
mod entities;

use sea_orm::{Database, DatabaseConnection};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL");
    println!("Database URL: {:?}", database_url);
    // Connect to the database and wrap it in Arc to share ownership
    let db_url = match database_url {
        Ok(ref url) => url,
        Err(e) => {
            panic!("DATABASE_URL: {}", e);
        }
    };
    let db: Arc<DatabaseConnection> = Arc::new(Database::connect(&*db_url).await.unwrap_or_else(|_| panic!("Error connecting to {:?}", database_url)));

    // Create and launch the Rocket application, managing the database connection
    rocket::build()
        .manage(db)
        .mount("/", routes::routes())
}
