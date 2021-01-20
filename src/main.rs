#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;


use std::fs::File;
use rocket::*;
use rocket_contrib::serve::StaticFiles;




#[get("/")]
pub fn index() -> File {
    File::open("./static/pages/index.html")
    .expect("File not found.")
} 

#[get("/commands")]
pub fn commands() -> File {
    File::open("./static/pages/commands.html")
    .expect("commands.html was not found")
}

fn rocket() -> Rocket {
    ignite()
    .mount("/static", StaticFiles::from("/styles"))
    .mount("/", routes![commands])
    .mount("/", routes![index])
}


fn main() {
    rocket()
    .launch();
}