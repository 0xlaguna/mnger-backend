use rocket::Route;

pub mod login;

pub fn account_routes() -> Vec<Route> {
    routes![login::req,]
}
