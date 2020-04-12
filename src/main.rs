use actix_web::{get, post, web, App, HttpServer, Responder};
use lettre::{ClientSecurity, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use serde::Deserialize;

#[derive(Deserialize)]
struct RequestBody {
    to: String,
    from: String,
    subject: String,
    html: String
}

#[get("/health")]
async fn health() -> impl Responder {
    format!("OK")
}

#[post("/api/v1/send")]
async fn send(body: web::Json<RequestBody>) -> impl Responder {
    let email = EmailBuilder::new()
        .to(body.to.clone())
        .from(body.from.clone())
        .subject(body.subject.clone())
        .html(body.html.clone())
        .build()
        .unwrap();

    let mut mailer = SmtpClient::new(("localhost", 1025), ClientSecurity::None)
        .unwrap()
        .smtp_utf8(true)
        .transport();

    let result = mailer.send(email.into());
    assert!(result.is_ok());
    mailer.close();

    format!("OK")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(send))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
