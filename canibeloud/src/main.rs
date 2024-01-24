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
    let target_element_id = "content";
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

        #{target_element_id} {{
            font-size: 15vw;
            text-align: center;
            padding: 10vw;
            border-radius: 10px;
        }}

        #response_text {{

        }}

        #secondary_text {{
            margin-top: 1em;
            font-size: 0.3em;
        }}

        #tz_datetime {{
            margin-top: 1em;
            font-size: 0.2em;
        }}

        .yes {{
            background-color: rgb(36, 138, 61);
            color: #f3f2f1;
        }}

        .no {{
            background-color: rgb(215, 0, 21);
            color: #f3f2f1;
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
                let parent_el = document.getElementById("{target_element_id}");
                parent_el.className = className;
                document.getElementById("response_text").innerHTML = data.response_text;
                document.getElementById("secondary_text").innerHTML = data.secondary_text;
                document.getElementById("tz_datetime").innerHTML = data.tz_datetime;
            }})
        }}
        can_i_be_loud();
    </script>
</head>
<body>
    <div id="{target_element_id}">
        <div id="response_text">
        </div>
        <div id="secondary_text">
        </div>
        <div id="tz_datetime">
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

    let rule_response = rule.can_i_be_loud(timezone.to_owned());
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
