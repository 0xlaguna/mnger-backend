#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

use altera::{Migrator, MigratorTrait};

use rocket_cors::{AllowedOrigins, CorsOptions};
use sea_orm_rocket::Database;
use std::str::FromStr;

use mnger_preon::r#impl::postgres::pool::Db;

pub mod routes;

// const DEFAULT_USERS_PER_PAGE: u64 = 5;


// #[post("/", data = "<user_form>")]
// async fn create(conn: Connection<'_, Db>, user_form: Form<user::Model>) -> Result<Json<user::Model>, Status> {
//     let db = conn.into_inner();

//     let form = user_form.into_inner();

//     if Mutation::create_user(db, form.clone()).await.is_err() {
//         return Err(Status::InternalServerError);
//     }

//     Ok(Json(form))
// }

// #[post("/<id>", data = "<user_form>")]
// async fn update(
//     conn: Connection<'_, Db>,
//     id: i32,
//     user_form: Form<user::Model>,
// ) -> Flash<Redirect> {
//     let db = conn.into_inner();

//     let form = user_form.into_inner();

//     Mutation::update_user_by_id(db, id, form)
//         .await
//         .expect("could not update user");

//     Flash::success(Redirect::to("/"), "User successfully edited.")
// }

// #[get("/?<page>&<users_per_page>")]
// async fn list(
//     conn: Connection<'_, Db>,
//     page: Option<u64>,
//     users_per_page: Option<u64>,
//     flash: Option<FlashMessage<'_>>,
// ) -> Template {
//     let db = conn.into_inner();

//     // Set page number and items per page
//     let page = page.unwrap_or(1);
//     let users_per_page = users_per_page.unwrap_or(DEFAULT_USERS_PER_PAGE);
//     if page == 0 {
//         panic!("Page number cannot be zero");
//     }

//     let (users, num_pages) = Query::find_users_in_page(db, page, users_per_page)
//         .await
//         .expect("Cannot find users in page");

//     Template::render(
//         "index",
//         json! ({
//             "page": page,
//             "users_per_page": users_per_page,
//             "num_pages": num_pages,
//             "users": users,
//             "flash": flash.map(FlashMessage::into_inner),
//         }),
//     )
// }

// #[get("/<id>")]
// async fn edit(conn: Connection<'_, Db>, id: i32) -> Template {
//     let db = conn.into_inner();

//     let user: Option<user::Model> = Query::find_user_by_id(db, id)
//         .await
//         .expect("could not find user");

//     Template::render(
//         "edit",
//         json! ({
//             "user": user,
//         }),
//     )
// }

// #[delete("/<id>")]
// async fn delete(conn: Connection<'_, Db>, id: i32) -> Flash<Redirect> {
//     let db = conn.into_inner();

//     Mutation::delete_user(db, id)
//         .await
//         .expect("could not delete User");

//     Flash::success(Redirect::to("/"), "User successfully deleted.")
// }

// #[delete("/")]
// async fn destroy(conn: Connection<'_, Db>) -> Result<(), rocket::response::Debug<String>> {
//     let db = conn.into_inner();

//     Mutation::delete_all_users(db)
//         .await
//         .map_err(|e| e.to_string())?;

//     Ok(())
// }

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = Migrator::up(conn, None).await;
    Ok(rocket)
}

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
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
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
