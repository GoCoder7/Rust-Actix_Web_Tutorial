use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(
                AppState {
                app_name: "root".to_owned(),
            }
        ))
        .service(index)
        .service(
            web::scope("sub")
            .app_data(web::Data::new(AppState {
                app_name: "sub".to_owned(),
            }))
            .service(index)
        )

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}