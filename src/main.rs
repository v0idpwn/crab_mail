use actix_web::{get, post, App, HttpServer, Responder};
use lettre::{ClientSecurity, SmtpClient, Transport};
use lettre_email::EmailBuilder;

#[get("/health")]
async fn health() -> impl Responder {
    format!("OK")
}

#[post("/api/v1/send")]
async fn send() -> impl Responder {
    let email = EmailBuilder::new()
        .to(("paleking@hallow.nest", "Pale King"))
        .from("theknight@example.com")
        .subject("Example email")
        .html("We got html support")
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
