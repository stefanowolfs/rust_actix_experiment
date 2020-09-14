use actix_web::{web, App, Error, HttpRequest, HttpResponse, Responder, HttpServer};
use serde::Serialize;
use futures::future::{ready, Ready};

#[derive(Serialize)]
struct BaseResponseDocument {
    name: &'static str,
}

impl Responder for BaseResponseDocument {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn index() -> impl Responder {
    BaseResponseDocument { name: "Stefano" }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/user", web::get().to(index)),
        )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}