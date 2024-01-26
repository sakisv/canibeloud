mod rules;
mod canibeloud;

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
    let answer = format!(r#"
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Noto+Sans+Mono:wght@900&family=Open+Sans:wght@800&family=Roboto+Mono:wght@100;400;500;700&display=swap" rel="stylesheet">
    <style>
        body {{
            font-family: 'Open Sans', sans-serif;
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
        }}

        #content {{
            font-size: 15vw;
            text-align: center;
            border-radius: 10px;
        }}

        #response_text {{
            font-family: 'Noto Sans Mono', monospace;
        }}

        #secondary_text {{
            margin-top: 1em;
            font-size: 0.3em;
        }}

        #tz_datetime {{
            margin-top: 1em;
            font-size: 0.2em;
        }}

        #source {{
            margin-top: 1em;
            font-size: 0.1em;
        }}

        a {{
            color: #f3f2f1;
        }}

        .yes {{
            background-color: rgb(36, 138, 61);
            color: #f3f2f1;
        }}

        .no {{
            background-color: rgb(215, 0, 21);
            color: #f3f2f1;
        }}

        .hidden {{
            display: none;
        }}

    </style>
    <script>
        function can_i_be_loud() {{
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
                document.body.className = className;
                document.getElementById("response_text").innerHTML = data.response_text.toUpperCase();
                document.getElementById("secondary_text").innerHTML = data.secondary_text;
                document.getElementById("tz_datetime").innerHTML = data.tz_datetime;
                document.getElementById("source").innerHTML = data.source != "" ? "<a href=\"" + data.source + "\">Source</a>" : "";
            }})
        }}
        can_i_be_loud();
    </script>
</head>
<body>
    <div id="content">
        <div id="response_text">
        </div>
        <div id="secondary_text">
        </div>
        <div id="tz_datetime">
        </div>
        <div id="source">
        </div>
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

    let utc_now = chrono::Utc::now();
    let rule_response = rule.can_i_be_loud(utc_now, timezone.to_owned());
    CanIBeLoudResponse {
        can_i_be_loud: rule_response.can_i_be_loud,
        response_text: rule_response.response_text,
        secondary_text: rule_response.secondary_text,
        requested_timezone: timezone.to_owned(),
        tz_datetime: rule_response.tz_datetime,
        source: rule_response.source_url,
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
