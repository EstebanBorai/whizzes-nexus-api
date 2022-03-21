use rocket::request::Request;

#[rocket::catch(404)]
pub fn not_found(request: &Request) -> String {
    format!(
        "Sorry! The page you are looking for {} doesn't exists!",
        request.uri()
    )
}
