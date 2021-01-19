#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
use std::fs::File;
use rocket::*;

#[get("/")]
fn index() -> File {
    File::open("./static/index.html").expect("Files not found.")
}


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}