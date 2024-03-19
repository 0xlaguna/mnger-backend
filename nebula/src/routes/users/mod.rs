use rocket::Route;

mod fetch_profile;
mod create_user;

pub fn user_routes() -> Vec<Route> {
    routes![
        fetch_profile::req,
        create_user::req,
    ]
}
