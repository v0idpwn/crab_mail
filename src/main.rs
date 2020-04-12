use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use actix_web::middleware::Logger;
use lettre::{ClientSecurity, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Deserialize)]
struct RequestBody {
    to: String,
    from: String,
    subject: String,
    html: String
}

#[derive(Serialize)]
struct Response {
    status: String,
    message: String
}

#[get("/health")]
async fn health() -> impl Responder {
    format!("OK")
}

#[post("/api/v1/send")]
async fn send(body: web::Json<RequestBody>) -> HttpResponse {
    let email = EmailBuilder::new()
        .to(body.to.clone())
        .from(body.from.clone())
        .subject(body.subject.clone())
        .html(body.html.clone())
        .build();

    let host = env::var("SMTP_HOST").expect("SMTP_HOST environment is mandatory");
    let port : u16 = env::var("SMTP_PORT").expect("SMTP_PORT environment is mandatory")
                    .parse::<u16>().expect("SMTP_PORT must be a number");

    let mut mailer = SmtpClient::new((host.as_str(), port), ClientSecurity::None)
        .unwrap()
        .smtp_utf8(true)
        .transport();

    let response = 
        match email {
            Ok(e) => {
              let result = mailer.send(e.into());
              match result.is_ok() {
                  true => ok_resp(),
                  false => error_resp("error on sending routine")
              }
            },
            Err(_) => error_resp("invalid params"),
        };

    mailer.close();

    response
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().service(health).service(send).wrap(Logger::default()))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

fn ok_resp() -> HttpResponse {
    HttpResponse::Ok().json(Response{
        status: "ok".to_string(),
        message: "email sent".to_string()
    })
}

fn error_resp(message: &str) -> HttpResponse {
    HttpResponse::Ok().json(Response {
        status: "error".to_string(),
        message: message.to_string()
    })
}
