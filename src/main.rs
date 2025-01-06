#[macro_use]
extern crate rocket;

mod routes;
mod models;

use rocket::fairing::AdHoc;
use rocket::{Ignite, Rocket};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build()
        .mount("/",routes![routes::index]);
    rocket.launch().await?;
    Ok(())
}