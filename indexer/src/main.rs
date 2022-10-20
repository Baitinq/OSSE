use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world! Im the indexer!");

    serve_http_endpoint("127.0.0.1", 4444).await
}

async fn serve_http_endpoint(address: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(add_resource))
        .bind((address, port))?
        .run()
        .await
}

#[derive(Deserialize, Debug)]
struct Resource {
    url: String,
    content: String,
}

#[post("/resource")]
async fn add_resource(resource: web::Json<Resource>) -> impl Responder {
    println!("Added resource! {:?}", resource);
    format!("{:?}", resource)
}

#[get("/search/{term}")]
async fn greet(term: web::Path<String>) -> impl Responder {
    format!("Searching for: {term}")
}
