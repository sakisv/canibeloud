mod rules;
use rules::rule_gr::RuleGR;
mod canibeloud;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    // 1. get country if available
    // 2. create instance of that country's law passing current timestamp
    // 3. get the can_i_be_loud result of that country
    // 4. print the message
    let rule = RuleGR::can_i_be_loud();

    let answer = format!("<html><body><p>{}</p></body></html>", rule.get_message());

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
