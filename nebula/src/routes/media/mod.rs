use rocket::Route;

pub mod fetch_avatar;

pub fn media_routes() -> Vec<Route> {
    routes![fetch_avatar::req]
}
