mod rules;
mod canibeloud;
use std::collections::HashMap;

use canibeloud::can_i_be_loud::CanIBeLoudResponse;

use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Result};
use rules::rule::Rulelike;
use serde::Deserialize;

#[derive(Deserialize)]
struct TimezoneFromRequest {
    timezone: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let targetElementId = "content";
    let answer = format!(r#"
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {{
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
        }}

        #{targetElementId} {{
            font-size: 15vw;
            text-align: center;
            padding: 10vw;
            border-radius: 10px;
        }}

        .yes {{
            background-color: green;
            color: white;
        }}

        .no {{
            background-color: red;
            color: white;
        }}
    </style>
    <script>
        function can_i_be_loud() {{
            console.log("boo");
            let origin = window.location.origin;
            let timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
            fetch(
                `${{origin}}/cibl`,
                {{
                    "method": "POST",
                    "headers": {{"Content-Type": "application/json"}},
                    "body": JSON.stringify({{"timezone": timezone}})
                }}
            ).then((response) => response.json())
            .then((data) => {{
                let className = data.can_i_be_loud ? "yes" : "no";
                let el = document.getElementById("{targetElementId}");
                el.className = className;
                el.innerHTML = data.response_text;
            }})
        }}
        can_i_be_loud();
    </script>
</head>
<body>
    <div id="{targetElementId}">
    </div>
</body>
</html>"#);

    HttpResponse::Ok().body(answer)
}

fn can_i_be_loud_from_tz(timezone: &str) -> CanIBeLoudResponse {
    let rule: Box<dyn Rulelike> = match timezone {
        "Europe/London" => Box::new(rules::europe_london::EuropeLondon{}),
        "Europe/Athens" => Box::new(rules::europe_athens::EuropeAthens{}),
        _ => Box::new(rules::rule::OtherTimezone{}),
    };

    let rule_response = rule.can_i_be_loud(timezone.to_owned());
    CanIBeLoudResponse {
        can_i_be_loud: rule_response.can_i_be_loud,
        response_text: rule_response.response_text,
        secondary_text: rule_response.secondary_text,
        requested_timezone: timezone.to_owned(),
        tz_datetime: rule_response.tz_datetime,
        timezone_found: false,
    }
}

async fn cibl(tz_from_request: web::Json<TimezoneFromRequest>) -> Result<impl Responder> {
    let response = can_i_be_loud_from_tz(&tz_from_request.timezone);

    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/cibl", web::post().to(cibl))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
