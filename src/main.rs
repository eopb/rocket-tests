#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use serde::{Deserialize, Serialize};

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
fn for_github_user(user: String) -> String {
    let resp: Vec<Repo> = reqwest::get(&format!("https://api.github.com/users/{}/repos", user))
        .unwrap()
        .json()
        .unwrap();
    format!("{:#?}", resp)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/github", routes![for_github_user])
        .mount("/public/style", StaticFiles::from("style"))
        .launch();
}
