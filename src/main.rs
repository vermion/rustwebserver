use actix_web::{web, App, HttpServer, HttpRequest, Responder, HttpResponse};

async fn index(req: HttpRequest) -> impl Responder {
    match req.headers().get("X-Real-IP") {
        Some(header_value) => {
            match header_value.to_str() {
                Ok(value) => format!("Your real IP address is: {}", value),
                Err(_) => "Invalid header value".to_string(),
            }
        },
        None => "X-Real-IP header not found".to_string(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("0.0.0.0:9101")?
    .run()
    .await
}
