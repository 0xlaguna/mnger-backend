#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

use rocket_cors::{AllowedOrigins, CorsOptions};
use sea_orm_rocket::Database;
use std::str::FromStr;

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use mnger_preon::r#impl::postgres::pool::Db;

pub mod routes;
pub mod docs;


#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    let rocket = rocket::build();

    // Configure CORS
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: [
            "Get", "Put", "Post", "Delete", "Options", "Head", "Trace", "Connect", "Patch",
        ]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect(),
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS.");

    routes::mount(rocket)
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", docs::ApiDoc::openapi()),
        )
        .mount("/", RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .mount("/", Redoc::with_url("/redoc", docs::ApiDoc::openapi()))
        .mount("/", Scalar::with_url("/scalar", docs::ApiDoc::openapi()))
        .attach(Db::init())
        .attach(Template::fairing())
        .manage(cors.clone())
        .launch()
        .await
        .map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
