#![feature(proc_macro_hygiene, decl_macro)]

mod routes;
use crate::routes::*;

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;


fn main() {
    ship()
    .launch();
}