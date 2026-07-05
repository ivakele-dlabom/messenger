use actix_web::{App, HttpResponse, HttpServer, Responder,  get, post, web::{self}};
use lettre::{ Message, SmtpTransport, Transport,  transport::smtp::authentication::Credentials};
use dotenv::dotenv;

use crate::config::Config;
mod mailer;
mod config;

// Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let config = Config::from_env()
        .expect("Failed to load configuration from environment");

    println!("Server starting on 127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(send_email)
            .service(hello)
            // ... other routes
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

#[post("/send-email")]
async fn send_email(config: web::Data<Config>) -> HttpResponse {
    // Build email
    let email = match Message::builder()
        .from(config.sender_email.as_str().parse().unwrap())
        .to(config.reciever_email.as_str().parse().unwrap())
        .subject("Test email")
        .body(String::from("Etsedfgidnfgoie efkonedfa"))
    {
        Ok(email) => email,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to build email: {}", e));
        }
    };

    // Create credentials
    let cred = Credentials::new(
        config.cred_email.clone(), 
        config.cred_app_password.replace(" ", "")
    );

    // Build mailer
    let mailer = match SmtpTransport::starttls_relay("smtp.gmail.com") {
        Ok(relay) => relay.credentials(cred).port(587).build(),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to build mailer: {}", e));
            }
};

    // Send email
    print!("Reicsdosd");
    match mailer.send(&email) {
        Ok(_) => HttpResponse::Ok().body("Email sent successfully!"),
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(format!("Could not send email: {:?}", e))
        }
    }
}




#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
