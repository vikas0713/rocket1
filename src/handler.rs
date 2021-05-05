use warp::{Reply, reject};
use warp::reply::html;
use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
struct WelcomeTemplate<'a> {
    title: &'a str,
    body: &'a str,
}

