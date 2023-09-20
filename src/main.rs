#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> String { 
    String::from("hello")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
