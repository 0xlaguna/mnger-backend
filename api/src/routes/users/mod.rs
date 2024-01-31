use rocket::Route;

mod fetch_profile;

pub fn user_routes() -> Vec<Route> {
    routes![fetch_profile::req,]
}
