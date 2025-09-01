use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// A user-friendly error response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub error_type: String,
    pub details: Option<serde_json::Value>,
}

/// Custom error types for the application
#[derive(Debug, Error)]
pub enum AppError {
    /// Storage-related errors
    #[error("Storage error: {0}")]
    Storage(String),

    /// Authentication-related errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),

    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl AppError {
    /// Create a user-friendly error response
    pub fn to_error_response(&self) -> ErrorResponse {
        match self {
            AppError::Storage(msg) => ErrorResponse {
                error: "Storage Error".to_string(),
                message: msg.clone(),
                error_type: "STORAGE_ERROR".to_string(),
                details: None,
            },
            AppError::Auth(msg) => ErrorResponse {
                error: "Authentication Error".to_string(),
                message: msg.clone(),
                error_type: "AUTH_ERROR".to_string(),
                details: None,
            },
            AppError::Validation(msg) => ErrorResponse {
                error: "Validation Error".to_string(),
                message: msg.clone(),
                error_type: "VALIDATION_ERROR".to_string(),
                details: None,
            },
            AppError::NotFound(msg) => ErrorResponse {
                error: "Not Found".to_string(),
                message: msg.clone(),
                error_type: "NOT_FOUND".to_string(),
                details: None,
            },
            AppError::Internal(msg) => ErrorResponse {
                error: "Internal Server Error".to_string(),
                message: "An unexpected error occurred. Please try again later.".to_string(),
                error_type: "INTERNAL_ERROR".to_string(),
                details: if cfg!(debug_assertions) {
                    Some(serde_json::json!({"debug_info": msg}))
                } else {
                    None
                },
            },
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = self.to_error_response();
        let status_code = match self {
            AppError::Storage(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Auth(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::Internal(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

/// Extension trait for Result to easily convert to AppError
pub trait ResultExt<T, E> {
    fn map_storage_err(self) -> Result<T, AppError>
    where
        E: fmt::Display;

    fn map_auth_err(self) -> Result<T, AppError>
    where
        E: fmt::Display;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn map_storage_err(self) -> Result<T, AppError>
    where
        E: fmt::Display,
    {
        self.map_err(|e| AppError::Storage(e.to_string()))
    }

    fn map_auth_err(self) -> Result<T, AppError>
    where
        E: fmt::Display,
    {
        self.map_err(|e| AppError::Auth(e.to_string()))
    }
}
