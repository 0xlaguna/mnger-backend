use rocket::Route;

mod entry;

pub fn index_routes() -> Vec<Route> {
    routes![entry::req,]
}
