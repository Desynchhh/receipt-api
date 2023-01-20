use actix_web::{ App, HttpServer };

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .configure(api::configure)
    })
    .bind(("127.0.0.1", 80))?
    .bind(("192.168.1.126", 80))?
    .run()
    .await
}
