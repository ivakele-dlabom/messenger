use std::result;

use actix_web::{HttpResponse, Responder, get, http::StatusCode, post, web::{self, Json}};
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use sqlx::{MySqlPool, pool};

use crate::{config::Config, models::{Bounds, Entry}};

#[post("/bounds/new")]
pub async fn insert_bounds(bounds: Json<Bounds>, pool: web::Data<MySqlPool>) -> HttpResponse {
    // create bounds table if not created yet
    let q = r#"CREATE TABLE IF NOT EXISTS bounds (
        id INT AUTO_INCREMENT PRIMARY KEY,
        x_min INT NOT NULL,
        x_max INT NOT NULL,
        y_min INT NOT NULL,
        y_max INT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#;
    let results = sqlx::query(q)
        .execute(pool.get_ref())
        .await;

    match results {
        Ok(_) => print!("Bounds create successfully"),
        Err(_) => {
            return HttpResponse::Ok().body("Error creating bounds");
        }
    }

    // Inserting the bounds
    let q = r#"
        INSERT INTO bounds (x_min, x_max, y_min, y_max) VALUES (?, ?, ?, ?)
        "#;

    let result = sqlx::query(q)
        .bind(bounds.x_min)
        .bind(bounds.x_max)
        .bind(bounds.y_min)
        .bind(bounds.x_max)
        .execute(pool.get_ref())
        .await
        ;

    match result {
        Ok(_) => HttpResponse::Created().body("Bound inserted successfully"),
        Err(_) => HttpResponse::BadRequest().body("Error inserting the Bound")
        
    }




}

#[get("/bounds")]
pub async fn get_bounds(pool: web::Data<MySqlPool>) -> HttpResponse {
    let result = sqlx::query!(
        r#"
        SELECT id, x_min, x_max, y_min, y_max 
        FROM bounds
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(rows) => {
            let bounds: Vec<Bounds> = rows
                .into_iter()
                .map(|row| Bounds {
                    x_min: row.x_min,
                    x_max: row.x_max,
                    y_min: row.y_min,
                    y_max: row.y_max,
                })
                .collect();
            HttpResponse::Ok().json(bounds)
        }
        Err(e) => {
            eprintln!("Error fetching bounds: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch bounds")
        }
    }
}

#[post("/entries")]
pub async fn register_entry(entry: web::Json<Entry>, pool: web::Data<MySqlPool>) -> HttpResponse {
    // create the entry table if ayikho
    let q = r#"
        CREATE TABLE IF NOT EXISTS entries (
            id INT AUTO_INCREMENT PRIMARY KEY,
            x INT NOT NULL,
            y INT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#;
    
    let create_table_result = sqlx::query(q)
        .execute(pool.get_ref())
        .await;
    
    match create_table_result {
        Err(_) => return HttpResponse::InternalServerError().body("Error creating the table"),
        Ok(_) => println!("table created"),
    }
    
    let result = sqlx::query(
        "INSERT INTO entries (x, y) VALUES (?, ?)"
    )
    .bind(entry.x)
    .bind(entry.y)
    .execute(pool.get_ref())
    .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().body("Entry registered successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error inserting entry"),
    }
}

#[post("/send-email")]
pub async fn send_email(config: web::Data<Config>) -> HttpResponse {
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


#[get("/entries")]
pub async fn get_entryies(pool: web::Data<MySqlPool>) -> HttpResponse {
    let res = sqlx::query!("SELECT * FROM entries")
    .fetch_all(pool.get_ref())
    .await;

match res {
    Ok(rows) => {
        for row in &rows {
            println!("Row data: {:?}", row);
        }
        HttpResponse::Ok().body("Went through")
    }
    Err(_) => HttpResponse::InternalServerError().body("Something went wrong")
}
}


#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

