use actix_web::{App, HttpResponse, HttpServer, Responder,  get, post, web::{self, Json}};
use lettre::{ Message, SmtpTransport, Transport,  transport::smtp::authentication::Credentials};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use actix_files::Files;
use actix_cors::Cors;
use sqlx::MySqlPool;
use crate::{config::Config, models::Entry, services::entry::{get_bounds, get_entryies, insert_bounds, register_entry, send_email}};

pub mod mailer;
pub mod config;
pub mod models;
pub mod services;





// Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let config = Config::from_env()
        .expect("Failed to load configuration from environment");


    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    println!("Server starting on 127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header(),
            )
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(send_email)
            .service(get_bounds)
            .service(insert_bounds)
            .service(get_entryies) 
            .service(register_entry)
            .service(
                Files::new("/", "./static")
                .index_file("index.html")
            )
            // ... other routes
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}


