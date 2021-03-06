use std::fs::File;
use rocket::*;
use rocket_contrib::serve::StaticFiles;

// Routes

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


pub fn ship() -> Rocket {
    ignite()
    .mount("/static", StaticFiles::from("/styles"))
    .mount("/", routes![commands])
    .mount("/", routes![index])
}