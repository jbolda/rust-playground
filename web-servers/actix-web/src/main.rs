use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::{get, web, App, HttpServer, Responder};
use async_stream::stream;
use std::convert::Infallible;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn response_body(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .streaming(stream! {
            yield Ok::<_, Infallible>(web::Bytes::from("Hello "));
            yield Ok::<_, Infallible>(web::Bytes::from(name));
            yield Ok::<_, Infallible>(web::Bytes::from("!"));
        })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet) // async response body
            .service(web::resource("/hello-again/{name}").route(web::get().to(response_body)))
            .service(web::resource("/hello-more/{name}").route(web::get().to(response_body)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
