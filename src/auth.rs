use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::error_handler::{AppError, ResultExt};
use crate::storage::{AdminUser, JsonStorage, StorageError};
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
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Password hashing error")]
    HashError,
}

impl From<AuthError> for AppError {
    fn from(auth_error: AuthError) -> Self {
        match auth_error {
            AuthError::Storage(storage_error) => AppError::Storage(storage_error.to_string()),
            AuthError::HashError => AppError::Auth("Password hashing error".to_string()),
        }
    }
}

/// Hash a password using Argon2 with secure parameters
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::HashError)?
        .to_string();
    Ok(password_hash)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| AuthError::HashError)?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    Ok(result.is_ok())
}

/// Login handler for POST /admin/login
pub async fn login_handler(
    storage: web::Data<JsonStorage>,
    session: Session,
    login_data: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    // Find user by username
    let user = storage
        .get_ref()
        .get_admin_user_by_username(&login_data.username)
        .map_storage_err()?
        .ok_or(AppError::Auth("Invalid username or password".to_string()))?;

    // Verify password
    if !verify_password(&login_data.password, &user.password_hash)? {
        return Err(AppError::Auth("Invalid username or password".to_string()));
    }

    // Set session
    log::debug!("Setting session for user: {}", user.username);
    session.insert("user_id", user.id).map_err(|e| {
        log::debug!("Error setting user_id in session: {:?}", e);
        AppError::Auth("Session error".to_string())
    })?;
    session.insert("username", user.username).map_err(|e| {
        log::debug!("Error setting username in session: {:?}", e);
        AppError::Auth("Session error".to_string())
    })?;

    session.renew();
    log::debug!("Session renewed successfully");

    // Debug: check if session values are set
    let check_user_id: Option<Uuid> = session.get("user_id").map_err(|e| {
        log::debug!("Error getting user_id for verification: {:?}", e);
        AppError::Auth("Session error".to_string())
    })?;
    let check_username: Option<String> = session.get("username").map_err(|e| {
        log::debug!("Error getting username for verification: {:?}", e);
        AppError::Auth("Session error".to_string())
    })?;
    log::debug!(
        "Session verification - user_id: {:?}, username: {:?}",
        check_user_id,
        check_username
    );

    let response = HttpResponse::SeeOther()
        .insert_header(("Location", "/admin"))
        .json(LoginResponse {
            message: "Login successful".to_string(),
            user_id: user.id,
        });

    log::debug!("Login response prepared with redirect");
    Ok(response)
}

/// Logout handler for POST /admin/logout
pub async fn logout_handler(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::SeeOther()
        .insert_header(("Location", "/admin/login"))
        .finish()
}

/// Middleware to protect admin routes
pub async fn require_auth(session: &Session) -> Result<Uuid, AppError> {
    log::debug!("require_auth() called");

    let user_id_result = session.get::<Uuid>("user_id");
    log::debug!("user_id result: {:?}", user_id_result);

    let user_id: Uuid = session
        .get("user_id")
        .map_err(|e| {
            log::debug!("Session error getting user_id: {:?}", e);
            AppError::Auth("Session error".to_string())
        })?
        .ok_or_else(|| {
            log::debug!("No user_id found in session");
            AppError::Auth("Invalid username or password".to_string())
        })?;

    log::debug!("User ID found: {}", user_id);
    Ok(user_id)
}

/// Create a default admin user if none exists
pub async fn create_default_admin(storage: web::Data<JsonStorage>) -> Result<(), AppError> {
    log::debug!("create_default_admin() started");

    log::debug!("Getting admin users list");
    let users = storage.get_ref().get_admin_users().map_storage_err()?;
    log::debug!("Found {} admin users", users.len());

    if users.is_empty() {
        log::debug!("No admin users found, creating default admin");
        log::debug!("Hashing password...");
        let password_hash = hash_password("admin123")?;
        log::debug!("Password hashed successfully");

        let admin_user = AdminUser {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            password_hash,
        };

        log::debug!("Adding admin user to storage on blocking thread");
        // Move the blocking storage operation to a dedicated thread
        let storage_clone = storage.clone();
        actix_rt::task::spawn_blocking(move || storage_clone.get_ref().add_admin_user(admin_user))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .map_storage_err()?;

        log::info!("Default admin user created: username='admin', password='admin123'");
    } else {
        log::debug!("Admin users already exist, skipping creation");
    }

    log::debug!("create_default_admin() completed successfully");
    Ok(())
}
