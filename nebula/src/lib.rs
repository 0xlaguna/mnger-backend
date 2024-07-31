#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

use sea_orm_rocket::Database;

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use mnger_preon::r#impl::postgres::pool::Db;

pub mod routes;
pub mod docs;
pub mod cors;


#[options("/<_..>")]
fn all_options() { }

#[tokio::main]
async fn start()  -> Result<(), rocket::Error> {
    let rocket = rocket::build()
        .attach(Db::init())
        .attach(Template::fairing())
        .attach(cors::Cors)
        .mount("/", routes![all_options])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", docs::ApiDoc::openapi()),
        )
        .mount("/", RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .mount("/", Redoc::with_url("/redoc", docs::ApiDoc::openapi()))
        .mount("/", Scalar::with_url("/scalar", docs::ApiDoc::openapi()));

    routes::mount(rocket)
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
