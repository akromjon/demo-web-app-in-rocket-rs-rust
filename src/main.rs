mod routes;
mod model;
mod controller;
#[macro_use] extern crate rocket;
use routes::web::routes;

#[launch]                            
fn rocket() -> _ {
    rocket::build().mount("/", routes())
}
