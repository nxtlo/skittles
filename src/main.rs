#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

#[macro_use]
extern crate serde_derive;


use std::fs::File;
use rocket::*;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> File {
    File::open("./static/index.html").expect("Files not found.")
}

#[get("/drop")]
fn drop() -> Json<String> {
    Json(String::from("Wdup"))
}

fn rocket() -> Rocket {
    ignite()
    .mount("/drop", routes![drop])
    .mount("/", routes![index])
}


fn main() {
    rocket()
    .launch();
}