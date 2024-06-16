use rocket::Route;

pub mod create_workorder;

pub fn workorder_routes() -> Vec<Route> {
    routes![
        create_workorder::req
    ]
}
