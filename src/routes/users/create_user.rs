use rocket::*;

#[post("/users")]
pub fn create_user() -> String {
    "Creation of new user".to_string()
}