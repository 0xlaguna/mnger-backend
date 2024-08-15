use rocket::Route;

pub mod fetch_profile;
pub mod create_user;
pub mod me;
pub mod edit_user;

pub fn user_routes() -> Vec<Route> {
    routes![
        fetch_profile::req,
        create_user::req,
        edit_user::req,
        me::req,
    ]
}
