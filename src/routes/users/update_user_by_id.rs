use rocket::*;

#[put("/users/<id>")]
pub fn update_user_by_id(id: String) -> String {
    format!("Update info for user {}", id)
}