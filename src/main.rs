#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
    let mut resp = json!({
        "repos": &reqwest::get(&format!("https://api.github.com/users/{}/repos", user))
                .unwrap()
                .json::<Vec<Repo>>()
                .unwrap(),
        "user": user
    });
    Template::render("user", &resp)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/github", routes![for_github_user])
        .attach(Template::fairing())
        .mount("/public/style", StaticFiles::from("style"))
        .launch();
} // serde_json::from_str(&serialized).unwrap();
