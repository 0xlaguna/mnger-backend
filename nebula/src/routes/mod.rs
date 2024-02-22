use rocket::{Build, Rocket};

mod users;
mod account;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/users", users::user_routes())
        .mount("/account", account::account_routes())
}
