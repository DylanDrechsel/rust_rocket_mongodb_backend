use rocket::*;

#[delete("/users/<id>")]
pub fn delete_user_by_id(id: String) -> String {
    format!("Delete user {}", id)
}