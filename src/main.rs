use std::env;

use actix_web::{App, Error, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use dotenv::dotenv;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct UserResponseDocument {
    id: &'static u32,
    username: &'static str,
}

#[derive(Deserialize)]
struct SearchRequestDocument {
    id: u32,
    username: String,
}

impl Responder for UserResponseDocument {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn user() -> impl Responder {
    let id: &u32 = &12354;
    let username: &str = "Stefano";

    UserResponseDocument {
        id,
        username,
    }
}

#[post("/search/{foo}/{bar}")]
async fn search(web::Path((foo, bar)): web::Path<(String, String)>,
                json: web::Json<SearchRequestDocument>,
) -> impl Responder {
    format!("id: {} | name: {} | foo: {} | bar: {}", json.id, json.username, foo, bar)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(index)
                .service(search)
                .route("/user", web::get().to(user)),
        )
    })
        .bind(server_url)?
        .run()
        .await
}