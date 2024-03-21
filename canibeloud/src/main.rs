use std::env;
use log::info;
mod rules;
mod canibeloud;

use canibeloud::can_i_be_loud::CanIBeLoudResponse;

use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::middleware::Logger;
use rules::rule::Rulelike;
use serde::Deserialize;

#[derive(Deserialize)]
struct TimezoneFromRequest {
    timezone: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let answer = include_str!("static/index.html");

    HttpResponse::Ok().body(answer)
}

fn can_i_be_loud_from_tz(timezone: &str) -> CanIBeLoudResponse {
    let rule: Box<dyn Rulelike> = match timezone {
        "Europe/London" => Box::new(rules::europe_london::EuropeLondon{}),
        "Europe/Athens" => Box::new(rules::europe_athens::EuropeAthens{}),
        "Europe/Zurich" => Box::new(rules::europe_zurich::EuropeZurich{}),
        _ => Box::new(rules::rule::OtherTimezone{}),
    };

    let utc_now = chrono::Utc::now();
    let rule_response = rule.can_i_be_loud(utc_now, timezone.to_owned());
    CanIBeLoudResponse {
        can_i_be_loud: rule_response.can_i_be_loud,
        response_text: rule_response.response_text,
        secondary_text: rule_response.secondary_text,
        requested_timezone: timezone.to_owned(),
        tz_datetime: rule_response.tz_datetime,
        source: rule_response.source_url,
        timezone_found: rule_response.tz_found,
    }
}

async fn cibl(tz_from_request: web::Json<TimezoneFromRequest>) -> Result<impl Responder> {
    let response = can_i_be_loud_from_tz(&tz_from_request.timezone);

    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = env::var("CAN_I_BE_LOUD_BIND_ADDR").unwrap_or("127.0.0.1".into());
    let port = env::var("CAN_I_BE_LOUD_PORT").unwrap_or("8080".into());
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Listening on {bind_addr}:{port}");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/cibl", web::post().to(cibl))
            .wrap(Logger::new(r#"%t %a "%{r}a" "%r" %s %b %Dms "%{Referer}i" "%{User-Agent}i""#))
    })
    .bind((bind_addr, port.parse().unwrap()))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::can_i_be_loud_from_tz;

    #[test]
    fn test_can_i_be_loud_from_tz() {
        let res = can_i_be_loud_from_tz("Europe/Athens");
        assert_eq!(true, res.timezone_found);

        let res = can_i_be_loud_from_tz("Asia/Beirut");
        assert_eq!(false, res.timezone_found);
    }
}
