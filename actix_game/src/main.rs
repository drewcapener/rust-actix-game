use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::cmp::Ordering;

static SECRET_NUMBER: u32 = 80;

#[get("/game")]
async fn game() -> impl Responder {
    HttpResponse::Ok().body("Input your guess by appending the query string guess to the guess endpoint (i.e. in the case of a guess of 5 /?guess=5).")
}

#[derive(Deserialize)]
struct Info {
    guess: u32,
}

#[get("/guess")]
async fn guess(info: web::Query<Info>) -> impl Responder {
    match info.guess.cmp(&SECRET_NUMBER) {
        Ordering::Less => HttpResponse::Ok().body(format!("{} is too small!", info.guess)),
        Ordering::Greater => HttpResponse::Ok().body(format!("{} is too big!", info.guess)),
        Ordering::Equal => HttpResponse::Ok().body(format!("{} is correct! You win!", info.guess))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(game)
            .service(guess)
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}