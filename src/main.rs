extern crate dotenv;

use std::env;

use warp::Filter;
use warp::hyper::Method;
use warp::http::header::HeaderMap;
use dotenv::dotenv;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let mut headers = HeaderMap::new();
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());

    let port = env::var("PORT")
        .unwrap_or_else(|_| {
            22333.to_string()
        })
        .parse()
        .unwrap();

    let routes = warp::get()
        .and(warp::path::end())
        .and(warp::fs::dir("www"))
        .with(warp::cors().allow_any_origin().allow_methods(&[Method::GET, Method::OPTIONS]))
        .with(warp::reply::with::headers(headers));
    
    println!("Server listening on port {}", port);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}