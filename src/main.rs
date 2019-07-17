#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world4!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/hello", routes![world])
        .launch();
}
