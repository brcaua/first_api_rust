// this is the library that allows to use the actix_web framework to build the API
use actix_web::*;

// import all the stuff from src/routes.rs
mod routes;
use routes::info::*;
use routes::ping::*;
use routes::products::*;

// this allows to define the main function as async
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api = HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/info", web::get().to(info))
            .route("/products", web::get().to(products))
    });

    let port = 9091;
    let test = api
        .bind(format!("127.0.0.1:{}", port))
        .expect("Can not bind to port 9091");

    println!("Server running");
    test.run().await
}
