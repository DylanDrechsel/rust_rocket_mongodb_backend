use rocket::*;

#[get("/users/<id>")]
pub fn get_user_by_id(id: String) -> String {
    format!("Info for user {}", id)
}