use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppStateWithCounter {
    count: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut count = data.count.lock().unwrap();
    *count += 1;
    format!("Request number: {count}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppStateWithCounter {
        count: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .service(index)
        .service(
            web::scope("sub")
            .app_data(data.clone())
            .service(index)
        )
    })
    .bind(("127.0.0.1", 8080))?
    // .workers(1)
    .run()
    .await
}