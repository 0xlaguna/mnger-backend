use rocket::Route;

pub mod create_workorder;
pub mod list_workorders;
pub mod fetch_workorder;

pub fn workorder_routes() -> Vec<Route> {
    routes![
        create_workorder::req,
        list_workorders::req,
        fetch_workorder::req,
    ]
}
