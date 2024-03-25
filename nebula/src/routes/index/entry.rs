use rocket_dyn_templates::{context, Template};
use sea_orm_rocket::Connection;
use mnger_preon::r#impl::postgres::pool::Db;

/// # Fetch index
#[get("/")]
pub async fn req(_conn: Connection<'_, Db>) -> Template {
    Template::render("index", context! {
        hi: "Hi",
    })
}
