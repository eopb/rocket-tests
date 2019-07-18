#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::Redirect;
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
fn index() -> Template {
    Template::render("index", &json!({}))
}

#[derive(FromForm)]
struct Submit {
    message: String,
}

#[post("/github", data = "<task>")]
fn submitted_user(task: Form<Submit>) -> Redirect {
    Redirect::to(format!("/github/{}", task.into_inner().message)) 
}

#[get("/<user>")]
fn for_github_user(user: String) -> Template {
    Template::render(
        "user",
        &json!({
            "repos": &reqwest::get(&format!("https://api.github.com/users/{}/repos", user))
                    .unwrap()
                    .json::<Vec<Repo>>()
                    .unwrap(),
            "user": user
        }),
    )
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![submitted_user])
        .mount("/github", routes![for_github_user])
        .mount("/public/style", StaticFiles::from("style"))
        .attach(Template::fairing())
        .launch();
} // serde_json::from_str(&serialized).unwrap();
