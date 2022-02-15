#[rocket::get("/")]
pub fn index() -> String {
    String::default()
}
