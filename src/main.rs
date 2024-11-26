mod controllers;

use actix_web::{web, App, HttpServer};
use controllers::sms::send_sms;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(send_sms)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}