use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::{Local, Datelike, Timelike};

#[get("/")]
async fn hello() -> impl Responder {
    let now = Local::now();
    // 1/10 - 31/3: 15:30-17:30 && 22:00-07:30
    // 1/4 - 30/9: 15:00-17:30 && 23:00-07:00
    // source:
    // https://www.astynomia.gr/odigos-tou-politi/chrisimes-symvoules/diafores/poies-einai-oi-ores-koinis-isychias/

    let no_answer = "<html><body><p>No</p></body></html>";
    let yes_answer = "<html><body><p>Yes (but within reason)</p></body></html>";
    match now.month() {
        4..=9 => {
            let start_noon = now.with_hour(15).unwrap().with_minute(0).unwrap();
            let stop_noon = now.with_hour(17).unwrap().with_minute(30).unwrap();

            let start_night = now.with_hour(23).unwrap().with_minute(0).unwrap();
            let end_night = now.with_hour(7).unwrap().with_minute(0).unwrap();
            if (now >= start_noon && now <= stop_noon) || (now >= start_night || now <= end_night) {
                return HttpResponse::Ok().body(no_answer);
            }
        }
        _ => {
            let start_noon = now.with_hour(15).unwrap().with_minute(30).unwrap();
            let stop_noon = now.with_hour(17).unwrap().with_minute(30).unwrap();

            let start_night = now.with_hour(22).unwrap().with_minute(0).unwrap();
            let end_night = now.with_hour(7).unwrap().with_minute(30).unwrap();
            if (now >= start_noon && now <= stop_noon) || (now >= start_night || now <= end_night) {
                return HttpResponse::Ok().body(no_answer);
            }
        }
    }

    HttpResponse::Ok().body(yes_answer)
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
