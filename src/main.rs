#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>/<age>")]
fn hello(name: String, age: String) -> String {
    format!("hi {} {}", name, age)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .launch();
}
