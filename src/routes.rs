#[get("/")]
pub fn index() -> &'static str {
    "Welcome to the Rocket App, Harsh Patel!"
}


pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}