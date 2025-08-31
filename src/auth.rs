use actix_web::{web, HttpResponse, Responder};
use actix_session::Session;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::storage::{JsonStorage, AdminUser, StorageError};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: Uuid,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Password hashing error")]
    HashError,
    #[error("Session error")]
    SessionError,
    #[error("Task join error: {0}")]
    JoinError(String),
}

impl actix_web::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            AuthError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::Storage(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::HashError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::SessionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::JoinError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        HttpResponse::build(status).json(crate::handlers::ApiError { error: self.to_string() })
    }
}

/// Hash a password using Argon2 with secure parameters
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::HashError)?
        .to_string();
    Ok(password_hash)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| AuthError::HashError)?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    Ok(result.is_ok())
}

/// Login handler for POST /admin/login
pub async fn login_handler(
    storage: web::Data<JsonStorage>,
    session: Session,
    login_data: web::Json<LoginRequest>,
) -> Result<impl Responder, AuthError> {
    // Find user by username
    let user = storage.get_ref().get_admin_user_by_username(&login_data.username)
        .map_err(AuthError::Storage)?
        .ok_or(AuthError::InvalidCredentials)?;

    // Verify password
    if !verify_password(&login_data.password, &user.password_hash)? {
        return Err(AuthError::InvalidCredentials);
    }

    // Set session
    println!("DEBUG: Setting session for user: {}", user.username);
    session.insert("user_id", user.id)
        .map_err(|e| {
            println!("DEBUG: Error setting user_id in session: {:?}", e);
            AuthError::SessionError
        })?;
    session.insert("username", user.username)
        .map_err(|e| {
            println!("DEBUG: Error setting username in session: {:?}", e);
            AuthError::SessionError
        })?;

    session.renew();
    println!("DEBUG: Session renewed successfully");

    // Debug: check if session values are set
    let check_user_id: Option<Uuid> = session.get("user_id").map_err(|e| {
        println!("DEBUG: Error getting user_id for verification: {:?}", e);
        AuthError::SessionError
    })?;
    let check_username: Option<String> = session.get("username").map_err(|e| {
        println!("DEBUG: Error getting username for verification: {:?}", e);
        AuthError::SessionError
    })?;
    println!("DEBUG: Session verification - user_id: {:?}, username: {:?}", check_user_id, check_username);

    let response = HttpResponse::SeeOther()
        .insert_header(("Location", "/admin"))
        .json(LoginResponse {
            message: "Login successful".to_string(),
            user_id: user.id,
        });

    println!("DEBUG: Login response prepared with redirect");
    Ok(response)
}

/// Logout handler for POST /admin/logout
pub async fn logout_handler(
    session: Session,
) -> impl Responder {
    session.purge();
    HttpResponse::Ok().json(crate::handlers::ApiError { error: "Logout successful".to_string() })
}

/// Middleware to protect admin routes
pub async fn require_auth(
    session: &Session,
) -> Result<Uuid, AuthError> {
    println!("DEBUG: require_auth() called");
    
    let user_id_result = session.get::<Uuid>("user_id");
    println!("DEBUG: user_id result: {:?}", user_id_result);
    
    let user_id: Uuid = session.get("user_id")
        .map_err(|e| {
            println!("DEBUG: Session error getting user_id: {:?}", e);
            AuthError::SessionError
        })?
        .ok_or_else(|| {
            println!("DEBUG: No user_id found in session");
            AuthError::InvalidCredentials
        })?;
        
    println!("DEBUG: User ID found: {}", user_id);
    Ok(user_id)
}

/// Create a default admin user if none exists
pub async fn create_default_admin(storage: web::Data<JsonStorage>) -> Result<(), AuthError> {
    println!("DEBUG: create_default_admin() started");
    
    println!("DEBUG: Getting admin users list");
    let users = storage.get_ref().get_admin_users().map_err(AuthError::Storage)?;
    println!("DEBUG: Found {} admin users", users.len());
    
    if users.is_empty() {
        println!("DEBUG: No admin users found, creating default admin");
        println!("DEBUG: Hashing password...");
        let password_hash = hash_password("admin123")?;
        println!("DEBUG: Password hashed successfully");
        
        let admin_user = AdminUser {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            password_hash,
        };
        
        println!("DEBUG: Adding admin user to storage on blocking thread");
        // Move the blocking storage operation to a dedicated thread
        let storage_clone = storage.clone();
        actix_rt::task::spawn_blocking(move || {
            storage_clone.get_ref().add_admin_user(admin_user)
        })
        .await
        .map_err(|e| AuthError::JoinError(e.to_string()))?
        .map_err(AuthError::Storage)?;
        
        println!("Default admin user created: username='admin', password='admin123'");
    } else {
        println!("DEBUG: Admin users already exist, skipping creation");
    }
    
    println!("DEBUG: create_default_admin() completed successfully");
    Ok(())
}