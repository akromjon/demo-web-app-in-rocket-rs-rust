use crate::controller::UserController;
use crate::model::user::User;
use rocket::serde::json::{self, Json};
use rocket::{get, Route};
use rocket::http::Status;

pub fn routes() -> Vec<Route> {
    routes![index, show]
}

#[get("/")]
pub fn index() -> Json<Vec<User>> {
    Json(UserController::index())
}
#[get("/<id>")]
pub fn show(id: u32) -> Json<Result<User, Status>> {
    
    match UserController::show(id) {
        Some(user) => Json(Ok(user)),   
        None => Json(Err(Status::NotFound)) 
    }
}