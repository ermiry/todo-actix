use actix_web::{ HttpServer, App, Responder, web };

async fn status() -> impl Responder {
    "{ \"status\": \"ok!\" }"
}

#[actix_web::main]
async fn main() -> std::io::Result <()> {
    println! ("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
