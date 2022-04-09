use rocket::*;

#[get("/users")]
pub fn get_users() -> String {
    "List of users".to_string()
}