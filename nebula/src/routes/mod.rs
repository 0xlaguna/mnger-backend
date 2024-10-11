use rocket::{Build, Rocket};

pub mod account;
pub mod index;
pub mod media;
pub mod users;
pub mod workorder;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", index::index_routes())
        .mount("/users", users::user_routes())
        .mount("/account", account::account_routes())
        .mount("/workorder", workorder::workorder_routes())
        .mount("/media", media::media_routes())
}
