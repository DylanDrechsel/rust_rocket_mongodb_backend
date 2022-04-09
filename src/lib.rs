#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use] use rocket::*;
use rocket_contrib::helmet::SpaceHelmet;

mod routes;

pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite().attach(SpaceHelmet::default())
    .mount("/", routes![
        routes::echo::echo_fn,
        routes::ping::ping_fn,
        routes::users::get_users::get_users,
        routes::users::create_user::create_user,
        routes::users::get_user_by_id::get_user_by_id,
        routes::users::update_user_by_id::update_user_by_id
    ])
}