#[macro_use]
extern crate rocket;


use rocket::{Build, Rocket};


mod database;
mod models;
mod schema;
mod controller;


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        controller::index,
        controller::delete_bird,
        controller::get_bird_by_id,
        controller::new_bird,
        controller::new_sighting
    ])
}
