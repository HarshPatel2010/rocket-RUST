#[get("/")]
pub fn index() -> &'static str {
    "Welcome to the Rocket App, Harsh!"
}
