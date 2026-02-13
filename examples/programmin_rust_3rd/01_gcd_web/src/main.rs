use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use actix_files::Files;
use serde::Deserialize;
mod gcd;
use gcd::gcd;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/gcd", web::post().to(post_gcd))
            .service(Files::new("/", "./static/root").index_file("index.html"))
    });
    log::info!("Server running at http://localhost:3000/");
    server
        .bind("127.0.0.1:3000")
        .expect("Failed to bind to address")
        .run()
        .await
        .expect("Running server failed");
}

async fn post_gcd(parameters: Option<web::Form<GcdParameters>>) -> HttpResponse {
    let parameters = match parameters {
        Some(p) => p,
        None => web::Form(GcdParameters {
            n: 0,
            m: 0,
        }),
    };
    let n = parameters.n;
    let m = parameters.m;
    log::info!("Calculating GCD of {} and {}", n, m);
    let result = gcd(n, m);
    HttpResponse::Ok().content_type("text/html").body(format!("The GCD of {} and {} is {}", n, m, result))
}