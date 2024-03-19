use rocket::Route;

mod login;

pub fn account_routes() -> Vec<Route> {
    routes![
        login::req,
    ]
}
