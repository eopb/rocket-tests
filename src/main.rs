#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Serialize)]
struct Repo {
    name: String,
    stargazers_count: u32,
    description: Option<String>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<user>")]
fn for_github_user(user: String) -> Template {
    let resp: Vec<Repo> = reqwest::get(&format!("https://api.github.com/users/{}/repos", user))
        .unwrap()
        .json()
        .unwrap();
    let mut context = HashMap::new();
    context.insert("user", user);
    context.insert("message", format!("{:#?}", resp));

    Template::render("user", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/github", routes![for_github_user])
        .attach(Template::fairing())
        .mount("/public/style", StaticFiles::from("style"))
        .launch();
}
