#![feature(proc_macro_hygiene, decl_macro)]
use askama::Template;
use warp::{Reply, reject};
use warp::reply::html;

mod handler;

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/url")]
fn home() -> &'static str {
    "New URL Defined for new method"
}

#[get("/index")]
fn welcome_handler() -> WebResult<impl Reply> {
    let template = WelcomeTemplate {
        title: "Welcome",
        body: "To The Bookstore!",
    };
    let res = template
        .render()
        .map_err(|e| reject::custom(TemplateError(e)))?;
    Ok(html(res))
}

fn main() {
    rocket::ignite().mount("/", routes![index, home]).launch();
}