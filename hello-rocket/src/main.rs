#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::RawStr;

#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

fn main() {
    rocket::ignite().mount("/hello", routes![world, hello]).launch();
}