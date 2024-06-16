use rocket::{Build, Rocket};

pub mod index;
pub mod users;
pub mod account;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", index::index_routes())
        .mount("/users", users::user_routes())
        .mount("/account", account::account_routes())
}
