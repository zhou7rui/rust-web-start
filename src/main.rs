use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

mod errors;
mod handler;
mod modle;
mod schema;

pub struct AppState {
    db: Pool<Postgres>,
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(
        r#"                       __                          ___.                       __                 __   
    _______ __ __  _______/  |_          __  _  __ ____\_ |__             _______/  |______ ________/  |_ 
    \_  __ \  |  \/  ___/\   __\  ______ \ \/ \/ // __ \| __ \   ______  /  ___/\   __\__  \\_  __ \   __\
     |  | \/  |  /\___ \  |  |   /_____/  \     /\  ___/| \_\ \ /_____/  \___ \  |  |  / __ \|  | \/|  |  
     |__|  |____//____  > |__|             \/\_/  \___  >___  /         /____  > |__| (____  /__|   |__|  
                      \/                              \/    \/               \/            \/             "#
    );
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let server_port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or("8000".to_string())
        .parse()
        .unwrap();

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!(
        "🚀 Server started successfully http://127.0.0.1:{}",
        server_port
    );

    //  etc.

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(health_checker_handler)
            .configure(handler::api_config)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}
