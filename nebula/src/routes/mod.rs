use rocket::{Build, Rocket};

mod users;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/users", users::user_routes())
}
