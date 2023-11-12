mod canibeloud;
use canibeloud::can_i_be_loud::CanIBeLoud;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::{Local, Datelike, Timelike};

fn can_i_be_loud() -> CanIBeLoud {
    // 1/10 - 31/3: 15:30-17:30 && 22:00-07:30
    // 1/4 - 30/9: 15:00-17:30 && 23:00-07:00
    // source:
    // https://www.astynomia.gr/odigos-tou-politi/chrisimes-symvoules/diafores/poies-einai-oi-ores-koinis-isychias/
    let now = Local::now();
    match now.month() {
        4..=9 => {
            let start_noon = now.with_hour(15).unwrap().with_minute(0).unwrap();
            let stop_noon = now.with_hour(17).unwrap().with_minute(30).unwrap();

            let start_night = now.with_hour(23).unwrap().with_minute(0).unwrap();
            let end_night = now.with_hour(7).unwrap().with_minute(0).unwrap();
            if (now >= start_noon && now <= stop_noon) || (now >= start_night || now <= end_night) {
                return CanIBeLoud::No;
            }
        }
        _ => {
            let start_noon = now.with_hour(15).unwrap().with_minute(30).unwrap();
            let stop_noon = now.with_hour(17).unwrap().with_minute(30).unwrap();

            let start_night = now.with_hour(22).unwrap().with_minute(0).unwrap();
            let end_night = now.with_hour(7).unwrap().with_minute(30).unwrap();
            if (now >= start_noon && now <= stop_noon) || (now >= start_night || now <= end_night) {
                return CanIBeLoud::No;
            }
        }
    }
    CanIBeLoud::Yes
}

#[get("/")]
async fn hello() -> impl Responder {
    let can_i_be_loud = can_i_be_loud();
    let answer = format!("<html><body><p>{}</p></body></html>", can_i_be_loud.get_message());

    HttpResponse::Ok().body(answer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
