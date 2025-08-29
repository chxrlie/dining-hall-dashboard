use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use tera::Tera;

use crate::storage::{JsonStorage, MenuItem, Notice, StorageError};
use crate::auth::require_auth;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Debug, Error)]
pub enum ApiErrorType {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Invalid input: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

impl actix_web::ResponseError for ApiErrorType {
    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        let status = match self {
            ApiErrorType::Storage(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorType::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiErrorType::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
        };

        HttpResponse::build(status).json(ApiError { error: error_message })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMenuItemRequest {
    pub name: String,
    pub category: String,
    pub description: String,
    pub allergens: Vec<String>,
    pub is_available: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMenuItemRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub allergens: Option<Vec<String>>,
    pub is_available: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoticeRequest {
    pub title: String,
    pub content: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoticeRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_active: Option<bool>,
}

// Menu Items Handlers

pub async fn list_menu_items(storage: web::Data<JsonStorage>) -> Result<impl Responder, ApiErrorType> {
    let items = storage.get_menu_items()
        .map_err(ApiErrorType::Storage)?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn create_menu_item(
    storage: web::Data<JsonStorage>,
    item_data: web::Json<CreateMenuItemRequest>,
) -> Result<impl Responder, ApiErrorType> {
    println!("DEBUG: create_menu_item() called with data: {:?}", item_data);
    // Validate category
    let category = match item_data.category.as_str() {
        "Mains" => crate::storage::MenuCategory::Mains,
        "Sides" => crate::storage::MenuCategory::Sides,
        "Desserts" => crate::storage::MenuCategory::Desserts,
        "Beverages" => crate::storage::MenuCategory::Beverages,
        _ => return Err(ApiErrorType::Validation("Invalid category".to_string())),
    };

    let new_item = MenuItem {
        id: Uuid::new_v4(),
        name: item_data.name.clone(),
        category,
        description: item_data.description.clone(),
        allergens: item_data.allergens.clone(),
        is_available: item_data.is_available,
    };

    println!("DEBUG: About to add menu item to storage: {:?}", new_item);
    storage.add_menu_item(new_item.clone())
        .map_err(ApiErrorType::Storage)?;
    println!("DEBUG: Menu item added to storage successfully");

    Ok(HttpResponse::Created().json(new_item))
}

pub async fn update_menu_item(
    storage: web::Data<JsonStorage>,
    path: web::Path<Uuid>,
    update_data: web::Json<UpdateMenuItemRequest>,
) -> Result<impl Responder, ApiErrorType> {
    let item_id = path.into_inner();
    
    // Get existing item
    let items = storage.get_menu_items()
        .map_err(ApiErrorType::Storage)?;
    let existing_item = items.iter()
        .find(|item| item.id == item_id)
        .ok_or_else(|| ApiErrorType::NotFound(format!("Menu item with id {} not found", item_id)))?;

    // Validate category if provided
    let category = if let Some(category_str) = &update_data.category {
        match category_str.as_str() {
            "Mains" => crate::storage::MenuCategory::Mains,
            "Sides" => crate::storage::MenuCategory::Sides,
            "Desserts" => crate::storage::MenuCategory::Desserts,
            "Beverages" => crate::storage::MenuCategory::Beverages,
            _ => return Err(ApiErrorType::Validation("Invalid category".to_string())),
        }
    } else {
        existing_item.category.clone()
    };

    let updated_item = MenuItem {
        id: item_id,
        name: update_data.name.clone().unwrap_or_else(|| existing_item.name.clone()),
        category,
        description: update_data.description.clone().unwrap_or_else(|| existing_item.description.clone()),
        allergens: update_data.allergens.clone().unwrap_or_else(|| existing_item.allergens.clone()),
        is_available: update_data.is_available.unwrap_or(existing_item.is_available),
    };

    storage.update_menu_item(item_id, updated_item.clone())
        .map_err(ApiErrorType::Storage)?;

    Ok(HttpResponse::Ok().json(updated_item))
}

pub async fn delete_menu_item(
    storage: web::Data<JsonStorage>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiErrorType> {
    let item_id = path.into_inner();
    
    storage.delete_menu_item(item_id)
        .map_err(ApiErrorType::Storage)?;

    Ok(HttpResponse::NoContent())
}

// Notices Handlers

pub async fn list_notices(storage: web::Data<JsonStorage>) -> Result<impl Responder, ApiErrorType> {
    let notices = storage.get_notices()
        .map_err(ApiErrorType::Storage)?;
    Ok(HttpResponse::Ok().json(notices))
}

pub async fn create_notice(
    storage: web::Data<JsonStorage>,
    notice_data: web::Json<CreateNoticeRequest>,
) -> Result<impl Responder, ApiErrorType> {
    use chrono::Utc;

    let new_notice = Notice {
        id: Uuid::new_v4(),
        title: notice_data.title.clone(),
        content: notice_data.content.clone(),
        is_active: notice_data.is_active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    storage.add_notice(new_notice.clone())
        .map_err(ApiErrorType::Storage)?;

    Ok(HttpResponse::Created().json(new_notice))
}

pub async fn update_notice(
    storage: web::Data<JsonStorage>,
    path: web::Path<Uuid>,
    update_data: web::Json<UpdateNoticeRequest>,
) -> Result<impl Responder, ApiErrorType> {
    let notice_id = path.into_inner();
    
    // Get existing notice
    let notices = storage.get_notices()
        .map_err(ApiErrorType::Storage)?;
    let existing_notice = notices.iter()
        .find(|notice| notice.id == notice_id)
        .ok_or_else(|| ApiErrorType::NotFound(format!("Notice with id {} not found", notice_id)))?;

    use chrono::Utc;

    let updated_notice = Notice {
        id: notice_id,
        title: update_data.title.clone().unwrap_or_else(|| existing_notice.title.clone()),
        content: update_data.content.clone().unwrap_or_else(|| existing_notice.content.clone()),
        is_active: update_data.is_active.unwrap_or(existing_notice.is_active),
        created_at: existing_notice.created_at,
        updated_at: Utc::now(),
    };

    storage.update_notice(notice_id, updated_notice.clone())
        .map_err(ApiErrorType::Storage)?;

    Ok(HttpResponse::Ok().json(updated_notice))
}

pub async fn delete_notice(
    storage: web::Data<JsonStorage>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiErrorType> {
    let notice_id = path.into_inner();
    
    storage.delete_notice(notice_id)
        .map_err(ApiErrorType::Storage)?;

    Ok(HttpResponse::NoContent())
}

// Login page handler
pub async fn login_page(
    tera: web::Data<Tera>,
) -> Result<HttpResponse, ApiErrorType> {
    println!("DEBUG: login_page handler called");
    
    let rendered = tera.render("admin/login.html", &tera::Context::new())
        .map_err(|e| ApiErrorType::Validation(format!("Template error: {}", e)))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// Admin Dashboard Handler
pub async fn admin_dashboard(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    tera: web::Data<Tera>,
) -> Result<HttpResponse, ApiErrorType> {
    println!("DEBUG: admin_dashboard handler called");
    
    // Check authentication
    let user_id = require_auth(&session).await.map_err(|e| {
        println!("DEBUG: Authentication failed in admin_dashboard: {}", e);
        ApiErrorType::Validation(format!("Authentication required: {}", e))
    })?;

    // Get menu items and notices
    let menu_items = storage.get_menu_items()
        .map_err(ApiErrorType::Storage)?;
    let notices = storage.get_notices()
        .map_err(ApiErrorType::Storage)?;

    // Prepare context for template
    let mut context = tera::Context::new();
    context.insert("menu_items", &menu_items);
    context.insert("notices", &notices);
    
    // Add session data to template context
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("session", &serde_json::json!({
            "username": username,
            "user_id": session.get::<Uuid>("user_id").ok().flatten()
        }));
    }

    // Render the template
    let rendered = tera.render("admin/dashboard.html", &context)
        .map_err(|e| ApiErrorType::Validation(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// Public Menu Display Handler
pub async fn menu_page(
    storage: web::Data<JsonStorage>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse, ApiErrorType> {
    println!("DEBUG: menu_page handler called");
    
    // Get menu items
    let menu_items = storage.get_menu_items()
        .map_err(ApiErrorType::Storage)?;
    
    // Prepare context for template
    let mut context = tera::Context::new();
    context.insert("menu_items", &menu_items);
    
    // Render the template
    let rendered = tera.render("menu.html", &context)
        .map_err(|e| ApiErrorType::Validation(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}