mod rules;
use canibeloud::can_i_be_loud::CanIBeLoud;
use rules::rule_gr::RuleGR;
mod canibeloud;

use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct TimezoneFromRequest {
    timezone: String,
}

#[derive(Serialize)]
struct CanIBeLoudResponse {
    can_i_be_loud: bool,
    response_text: String,
    requested_timezone: String,
    timezone_found: bool,
    calculated_datetime: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    // 1. get country if available
    // 2. create instance of that country's law passing current timestamp
    // 3. get the can_i_be_loud result of that country
    // 4. print the message
    let rule = RuleGR::can_i_be_loud();

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

fn can_i_be_loud_from_tz(timezone: String) -> CanIBeLoudResponse {
    CanIBeLoudResponse {
        can_i_be_loud: true,
        response_text: "yes".to_owned(),
        requested_timezone: timezone.to_owned(),
        timezone_found: true,
        calculated_datetime: "2023-10-10 19:55:55".to_owned()
    }
}

async fn cibl(tz_from_request: web::Json<TimezoneFromRequest>) -> Result<impl Responder> {
    let response = CanIBeLoudResponse {
        can_i_be_loud: true,
        response_text: "yes".to_owned(),
        requested_timezone: tz_from_request.timezone.to_owned(),
        timezone_found: true,
        calculated_datetime: "2023-10-10 19:55:55".to_owned()
    };

    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/cibl", web::post().to(cibl))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
