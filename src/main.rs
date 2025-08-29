mod auth;
mod handlers;
mod storage;

use std::error::Error;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_files::Files;
use actix_cors::Cors;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use storage::JsonStorage;
use tera::Tera;
use crate::auth::create_default_admin;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();
    println!("DEBUG: Starting main function");

    println!("Initializing JSON storage system...");
    println!("DEBUG: About to call JsonStorage::new()");

    // Initialize storage with file paths
    let storage = JsonStorage::new("data/menu_items.json", "data/notices.json", "data/admin_users.json")?;
    println!("DEBUG: JsonStorage::new() completed successfully");
    println!("Storage initialized successfully!");

    // Wrap storage in web::Data for Actix-web
    println!("DEBUG: Wrapping storage in web::Data");
    let storage_data = web::Data::new(storage);
    println!("DEBUG: Storage wrapped successfully");

    // Create default admin user if none exists
    println!("DEBUG: About to call create_default_admin()");
    create_default_admin(storage_data.clone()).await?;
    println!("DEBUG: create_default_admin() completed successfully");

    // Initialize Tera templates
    println!("DEBUG: Initializing Tera templates");
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera templates");
    let tera_data = web::Data::new(tera);
    println!("DEBUG: Tera templates initialized");

    // Create session key (in production, use a proper persistent secret key)
    // For development, use a fixed key to maintain sessions across restarts
    let secret_key = Key::from(&[0; 64]); // Fixed key for development
    println!("DEBUG: Using fixed session key for development");

    println!("DEBUG: About to configure HttpServer");
    println!("Starting Actix-web server on http://localhost:8080");

    HttpServer::new(move || {
        println!("DEBUG: Inside HttpServer closure");
        App::new()
            .app_data(storage_data.clone())
            .app_data(tera_data.clone())
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // Set to true in production with HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(actix_web::cookie::SameSite::Lax)
                    .cookie_path("/".to_string()) // Ensure cookie is sent for all paths
                    .cookie_domain(None) // Let browser determine domain
                    .build()
            )
            // Configure CORS
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
            )
            // Menu items routes
            .route("/api/items", web::get().to(handlers::list_menu_items))
            .route("/api/items", web::post().to(handlers::create_menu_item))
            .route("/api/items/{id}", web::put().to(handlers::update_menu_item))
            .route("/api/items/{id}", web::delete().to(handlers::delete_menu_item))
            // Notices routes
            .route("/api/notices", web::get().to(handlers::list_notices))
            .route("/api/notices", web::post().to(handlers::create_notice))
            .route("/api/notices/{id}", web::put().to(handlers::update_notice))
            .route("/api/notices/{id}", web::delete().to(handlers::delete_notice))
            // Authentication routes
            .route("/admin/login", web::post().to(auth::login_handler))
            .route("/admin/login", web::get().to(handlers::login_page))
            .route("/admin/logout", web::post().to(auth::logout_handler))
            // Admin dashboard route
            .route("/admin", web::get().to(handlers::admin_dashboard))
            // Serve static files
            .service(Files::new("/static", "./static").show_files_listing())
            // Public menu page
            .route("/menu", web::get().to(handlers::menu_page))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    println!("DEBUG: Server started successfully");

    Ok(())
}
