use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(||{HttpResponse::Ok().finish()}))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn health_check() -> HttpResponse{
    web::HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use crate:: health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}