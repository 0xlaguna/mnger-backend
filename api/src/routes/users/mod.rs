use revolt_rocket_okapi::revolt_okapi::openapi3::OpenApi;
use rocket::Route;

mod fetch_profile;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        fetch_profile::req,
    ]
}
