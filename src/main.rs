#![deny(warnings)]

extern crate dotenv;

use std::env;
use std::sync::Arc;

use warp::Filter;
use warp::http::header::HeaderMap;
use warp::hyper::{Method, StatusCode};
use serde::Serialize;
use dotenv::dotenv;
use handlebars::Handlebars;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply where T: Serialize, {

    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|error| error.to_string());

    warp::reply::html(render)
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let mut headers = HeaderMap::new();
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());

    let mut handlebars = Handlebars::new();
    // Register a template from a path on file system
    handlebars.register_template_file("index.hbs", "www/index.hbs").unwrap();

    let handlebars = Arc::new(handlebars);
    let template = move |template| render(template, handlebars.clone());

    let port = env::var("PORT")
        // Use default PORT if env is empty
        .unwrap_or("22333".into())
        .parse()
        .unwrap();

    let index = warp::path::end()
        .map(|| WithTemplate {name:"index.hbs", value: "" })
        .map(template);
    let robots = warp::path("robots.txt")
        .and(warp::fs::file("www/robots.txt"))
        .and(warp::path::end());
    let sitemap = warp::path("sitemap.xml")
        .and(warp::fs::file("www/sitemap.xml"))
        .and(warp::path::end());
    let error_filter = warp::any()
        .and(warp::path::full().map(|path| format!("Page {:?} cannot be found.", path)))
        .map(|message| warp::reply::with_status(message, StatusCode::NOT_FOUND));

    let routes = warp::get()
        .and(
            index
                .or(robots)
                .or(sitemap)
                .or(error_filter)
        )
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_methods(&[Method::GET, Method::OPTIONS])
        )
        .with(
            warp::reply::with::headers(headers)
        );

    println!("Server listening on port {}", port);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}