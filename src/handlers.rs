use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tera::Tera;

use crate::storage::{JsonStorage, MenuItem, Notice, MenuPreset, MenuSchedule, ScheduleRecurrence, ScheduleStatus, StorageError};
use crate::auth::require_auth;
use crate::error_handler::{AppError, ResultExt};

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiErrorType {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Invalid input: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<AppError> for ApiErrorType {
    fn from(app_error: AppError) -> Self {
        match app_error {
            AppError::Storage(msg) => ApiErrorType::Storage(StorageError::Io(std::io::Error::new(std::io::ErrorKind::Other, msg))),
            AppError::Auth(msg) => ApiErrorType::Validation(format!("Auth error: {}", msg)),
            AppError::Validation(msg) => ApiErrorType::Validation(msg),
            AppError::NotFound(msg) => ApiErrorType::NotFound(msg),
            AppError::Internal(msg) => ApiErrorType::Validation(format!("Internal error: {}", msg)),
        }
    }
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

#[derive(Debug, Deserialize)]
pub struct CreateMenuPresetRequest {
    pub name: String,
    pub description: String,
    pub menu_item_ids: Vec<uuid::Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMenuPresetRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub menu_item_ids: Option<Vec<uuid::Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMenuScheduleRequest {
    pub preset_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub recurrence: String, // Will be converted to ScheduleRecurrence enum
    pub status: String,     // Will be converted to ScheduleStatus enum
}

#[derive(Debug, Deserialize)]
pub struct UpdateMenuScheduleRequest {
    pub preset_id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub recurrence: Option<String>, // Will be converted to ScheduleRecurrence enum
    pub status: Option<String>,     // Will be converted to ScheduleStatus enum
}

#[derive(Debug, Deserialize)]
pub struct ValidateScheduleRequest {
    pub preset_id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub recurrence: Option<String>,
    pub status: Option<String>,
    pub schedule_id: Option<Uuid>, // For update validation
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
        .map_err(ApiErrorType::from)?;
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
        .map_err(ApiErrorType::from)?;

    Ok(HttpResponse::Ok().json(updated_item))
}

pub async fn delete_menu_item(
    storage: web::Data<JsonStorage>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiErrorType> {
    let item_id = path.into_inner();
    
    storage.delete_menu_item(item_id)
        .map_err(ApiErrorType::from)?;

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
        .map_err(ApiErrorType::from)?;

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
        .map_err(ApiErrorType::from)?;

    Ok(HttpResponse::Ok().json(updated_notice))
}

pub async fn delete_notice(
    storage: web::Data<JsonStorage>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiErrorType> {
    let notice_id = path.into_inner();
    
    storage.delete_notice(notice_id)
        .map_err(ApiErrorType::from)?;

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
    let _user_id = require_auth(&session).await.map_err(|e| {
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

// Menu Presets Handlers

pub async fn list_menu_presets(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
) -> Result<impl Responder, ApiErrorType> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        ApiErrorType::Validation(format!("Authentication required: {}", e))
    })?;
    
    let presets = storage.get_menu_presets()
        .map_err(ApiErrorType::Storage)?;
    Ok(HttpResponse::Ok().json(presets))
}

pub async fn create_menu_preset(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    preset_data: web::Json<CreateMenuPresetRequest>,
) -> Result<impl Responder, ApiErrorType> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        ApiErrorType::Validation(format!("Authentication required: {}", e))
    })?;
    
    use chrono::Utc;

    // Validate that menu item IDs exist
    let menu_items = storage.get_menu_items()
        .map_err(ApiErrorType::Storage)?;
    
    for item_id in &preset_data.menu_item_ids {
        if !menu_items.iter().any(|item| &item.id == item_id) {
            return Err(ApiErrorType::Validation(
                format!("Menu item with id {} not found", item_id)
            ));
        }
    }

    let new_preset = MenuPreset {
        id: Uuid::new_v4(),
        name: preset_data.name.clone(),
        description: preset_data.description.clone(),
        menu_item_ids: preset_data.menu_item_ids.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    storage.add_menu_preset(new_preset.clone())
        .map_err(ApiErrorType::Storage)?;

    Ok(HttpResponse::Created().json(new_preset))
}

pub async fn get_menu_preset(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiErrorType> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        ApiErrorType::Validation(format!("Authentication required: {}", e))
    })?;
    
    let preset_id = path.into_inner();
    
    let presets = storage.get_menu_presets()
        .map_err(ApiErrorType::Storage)?;
    
    let preset = presets.into_iter()
        .find(|p| p.id == preset_id)
        .ok_or_else(|| ApiErrorType::NotFound(
            format!("Menu preset with id {} not found", preset_id)
        ))?;

    Ok(HttpResponse::Ok().json(preset))
}

pub async fn update_menu_preset(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    path: web::Path<Uuid>,
    update_data: web::Json<UpdateMenuPresetRequest>,
) -> Result<impl Responder, ApiErrorType> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        ApiErrorType::Validation(format!("Authentication required: {}", e))
    })?;
    
    use chrono::Utc;
    
    let preset_id = path.into_inner();
    
    // Get existing preset
    let presets = storage.get_menu_presets()
        .map_err(ApiErrorType::Storage)?;
    
    let mut existing_preset = presets.into_iter()
        .find(|p| p.id == preset_id)
        .ok_or_else(|| ApiErrorType::NotFound(
            format!("Menu preset with id {} not found", preset_id)
        ))?;

    // Validate menu item IDs if provided
    if let Some(menu_item_ids) = &update_data.menu_item_ids {
        let menu_items = storage.get_menu_items()
            .map_err(ApiErrorType::Storage)?;
        
        for item_id in menu_item_ids {
            if !menu_items.iter().any(|item| &item.id == item_id) {
                return Err(ApiErrorType::Validation(
                    format!("Menu item with id {} not found", item_id)
                ));
            }
        }
        existing_preset.menu_item_ids = menu_item_ids.clone();
    }

    // Update fields
    if let Some(name) = &update_data.name {
        existing_preset.name = name.clone();
    }
    if let Some(description) = &update_data.description {
        existing_preset.description = description.clone();
    }
    existing_preset.updated_at = Utc::now();

    storage.update_menu_preset(preset_id, existing_preset.clone())
        .map_err(ApiErrorType::Storage)?;

    Ok(HttpResponse::Ok().json(existing_preset))
}

pub async fn delete_menu_preset(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        AppError::Validation(format!("Authentication required: {}", e))
    })?;
    
    let preset_id = path.into_inner();
    
    storage.delete_menu_preset(preset_id)
        .map_err(|e| AppError::from(e))?;

    Ok(HttpResponse::NoContent())
}

// Menu Schedules Handlers

pub async fn list_menu_schedules(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
) -> Result<impl Responder, AppError> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        AppError::Validation(format!("Authentication required: {}", e))
    })?;
    
    let schedules = storage.get_menu_schedules()
        .map_err(|e| AppError::from(e))?;
    Ok(HttpResponse::Ok().json(schedules))
}

pub async fn create_menu_schedule(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    schedule_data: web::Json<CreateMenuScheduleRequest>,
) -> Result<impl Responder, AppError> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        AppError::Validation(format!("Authentication required: {}", e))
    })?;
    
    use chrono::Utc;

    // Validate that preset exists
    let presets = storage.get_menu_presets()
        .map_err(|e| AppError::from(e))?;
    
    if !presets.iter().any(|preset| preset.id == schedule_data.preset_id) {
        return Err(AppError::Validation(
            format!("Menu preset with id {} not found", schedule_data.preset_id)
        ));
    }

    // Convert recurrence string to enum
    let recurrence = match schedule_data.recurrence.as_str() {
        "Daily" => ScheduleRecurrence::Daily,
        "Weekly" => ScheduleRecurrence::Weekly,
        "Monthly" => ScheduleRecurrence::Monthly,
        "Custom" => ScheduleRecurrence::Custom,
        _ => return Err(AppError::Validation("Invalid recurrence value".to_string())),
    };

    // Convert status string to enum
    let status = match schedule_data.status.as_str() {
        "Active" => ScheduleStatus::Active,
        "Inactive" => ScheduleStatus::Inactive,
        "Pending" => ScheduleStatus::Pending,
        _ => return Err(AppError::Validation("Invalid status value".to_string())),
    };

    // Check for schedule conflicts
    let existing_schedules = storage.get_menu_schedules()
        .map_err(|e| AppError::from(e))?;
    
    for schedule in existing_schedules {
        if schedule.preset_id == schedule_data.preset_id &&
           ((schedule.start_time <= schedule_data.start_time && schedule.end_time >= schedule_data.start_time) ||
            (schedule.start_time <= schedule_data.end_time && schedule.end_time >= schedule_data.end_time) ||
            (schedule.start_time >= schedule_data.start_time && schedule.end_time <= schedule_data.end_time)) {
            return Err(AppError::Validation(
                format!("Schedule conflict with existing schedule id {}", schedule.id)
            ));
        }
    }

    let new_schedule = MenuSchedule {
        id: Uuid::new_v4(),
        preset_id: schedule_data.preset_id,
        name: schedule_data.name.clone(),
        description: schedule_data.description.clone(),
        start_time: schedule_data.start_time,
        end_time: schedule_data.end_time,
        recurrence,
        status,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    storage.add_menu_schedule(new_schedule.clone())
        .map_err(|e| AppError::from(e))?;

    Ok(HttpResponse::Created().json(new_schedule))
}

pub async fn get_menu_schedule(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiErrorType> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        ApiErrorType::Validation(format!("Authentication required: {}", e))
    })?;
    
    let schedule_id = path.into_inner();
    
    let schedules = storage.get_menu_schedules()
        .map_err(ApiErrorType::Storage)?;
    
    let schedule = schedules.into_iter()
        .find(|s| s.id == schedule_id)
        .ok_or_else(|| ApiErrorType::NotFound(
            format!("Menu schedule with id {} not found", schedule_id)
        ))?;

    Ok(HttpResponse::Ok().json(schedule))
}

pub async fn update_menu_schedule(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    path: web::Path<Uuid>,
    update_data: web::Json<UpdateMenuScheduleRequest>,
) -> Result<impl Responder, AppError> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_auth_err()?;
    
    use chrono::Utc;
    
    let schedule_id = path.into_inner();
    
    // Get existing schedule
    let schedules = storage.get_menu_schedules().map_storage_err()?;
    
    let mut existing_schedule = schedules.into_iter()
        .find(|s| s.id == schedule_id)
        .ok_or_else(|| AppError::NotFound(
            format!("Menu schedule with id {} not found", schedule_id)
        ))?;

    // Validate preset_id if provided
    if let Some(preset_id) = update_data.preset_id {
        let presets = storage.get_menu_presets().map_storage_err()?;
        
        if !presets.iter().any(|preset| preset.id == preset_id) {
            return Err(AppError::Validation(
                format!("Menu preset with id {} not found", preset_id)
            ));
        }
        existing_schedule.preset_id = preset_id;
    }

    // Update fields
    if let Some(name) = &update_data.name {
        existing_schedule.name = name.clone();
    }
    if let Some(description) = &update_data.description {
        existing_schedule.description = description.clone();
    }
    if let Some(start_time) = update_data.start_time {
        existing_schedule.start_time = start_time;
    }
    if let Some(end_time) = update_data.end_time {
        existing_schedule.end_time = end_time;
    }
    
    // Convert recurrence string to enum if provided
    if let Some(recurrence_str) = &update_data.recurrence {
        let recurrence = match recurrence_str.as_str() {
            "Daily" => ScheduleRecurrence::Daily,
            "Weekly" => ScheduleRecurrence::Weekly,
            "Monthly" => ScheduleRecurrence::Monthly,
            "Custom" => ScheduleRecurrence::Custom,
            _ => return Err(AppError::Validation("Invalid recurrence value".to_string())),
        };
        existing_schedule.recurrence = recurrence;
    }
    
    // Convert status string to enum if provided
    if let Some(status_str) = &update_data.status {
        let status = match status_str.as_str() {
            "Active" => ScheduleStatus::Active,
            "Inactive" => ScheduleStatus::Inactive,
            "Pending" => ScheduleStatus::Pending,
            _ => return Err(AppError::Validation("Invalid status value".to_string())),
        };
        existing_schedule.status = status;
    }
    
    existing_schedule.updated_at = Utc::now();

    storage.update_menu_schedule(schedule_id, existing_schedule.clone()).map_storage_err()?;

    Ok(HttpResponse::Ok().json(existing_schedule))
}

pub async fn delete_menu_schedule(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_auth_err()?;
    
    let schedule_id = path.into_inner();
    
    storage.delete_menu_schedule(schedule_id).map_storage_err()?;

    Ok(HttpResponse::NoContent())
}

pub async fn get_upcoming_schedules(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
) -> Result<impl Responder, AppError> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_auth_err()?;
    
    use chrono::Utc;
    
    let schedules = storage.get_menu_schedules().map_storage_err()?;
    
    // Filter for upcoming schedules (start time is in the future)
    let upcoming_schedules: Vec<MenuSchedule> = schedules.into_iter()
        .filter(|schedule| schedule.start_time > Utc::now())
        .collect();

    Ok(HttpResponse::Ok().json(upcoming_schedules))
}

pub async fn validate_schedule(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    validation_data: web::Json<ValidateScheduleRequest>,
) -> Result<impl Responder, AppError> {
    // Check authentication
    let _user_id = require_auth(&session).await.map_auth_err()?;
    
    // Validate that end time is after start time
    if validation_data.end_time <= validation_data.start_time {
        return Err(AppError::Validation(
            "End time must be after start time".to_string()
        ));
    }
    
    // Validate preset exists if provided
    if let Some(preset_id) = validation_data.preset_id {
        let presets = storage.get_menu_presets().map_storage_err()?;
        
        if !presets.iter().any(|preset| preset.id == preset_id) {
            return Err(AppError::Validation(
                format!("Menu preset with id {} not found", preset_id)
            ));
        }
    }
    
    // Validate name if provided
    if let Some(name) = &validation_data.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Schedule name cannot be empty".to_string()
            ));
        }
    }
    
    // Validate description if provided
    if let Some(description) = &validation_data.description {
        if description.trim().is_empty() {
            return Err(AppError::Validation(
                "Schedule description cannot be empty".to_string()
            ));
        }
    }
    
    // Validate recurrence if provided
    if let Some(recurrence) = &validation_data.recurrence {
        match recurrence.as_str() {
            "Daily" | "Weekly" | "Monthly" | "Custom" => {},
            _ => return Err(AppError::Validation(
                "Invalid recurrence value".to_string()
            )),
        }
    }
    
    // Validate status if provided
    if let Some(status) = &validation_data.status {
        match status.as_str() {
            "Active" | "Inactive" | "Pending" => {},
            _ => return Err(AppError::Validation(
                "Invalid status value".to_string()
            )),
        }
    }
    
    // Check for schedule conflicts
    let existing_schedules = storage.get_menu_schedules().map_storage_err()?;
    
    let mut conflicts = Vec::new();
    let schedule_id = validation_data.schedule_id;
    
    for schedule in existing_schedules {
        // Skip the schedule being updated
        if let Some(id) = schedule_id {
            if schedule.id == id {
                continue;
            }
        }
        
        // Check for time overlap
        if (schedule.start_time <= validation_data.start_time && schedule.end_time >= validation_data.start_time) ||
           (schedule.start_time <= validation_data.end_time && schedule.end_time >= validation_data.end_time) ||
           (schedule.start_time >= validation_data.start_time && schedule.end_time <= validation_data.end_time) {
            // If preset_id is provided, only check conflicts with schedules that use the same preset
            if let Some(preset_id) = validation_data.preset_id {
                if schedule.preset_id == preset_id {
                    conflicts.push(schedule.id);
                }
            } else {
                conflicts.push(schedule.id);
            }
        }
    }

    #[derive(Debug, Serialize)]
    struct ValidationResponse {
        is_valid: bool,
        conflicts: Vec<Uuid>,
        message: Option<String>,
    }

    let has_conflicts = !conflicts.is_empty();
    let response = ValidationResponse {
        is_valid: !has_conflicts,
        conflicts,
        message: if has_conflicts {
            Some("Schedule conflicts with existing schedules".to_string())
        } else {
            None
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

// Public Menu Display Handler
pub async fn menu_page(
    storage: web::Data<JsonStorage>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse, ApiErrorType> {
    println!("DEBUG: menu_page handler called");
    
    // Get menu items and filter for available ones
    let menu_items = storage.get_menu_items()
        .map_err(ApiErrorType::Storage)?;
    let available_menu_items: Vec<&MenuItem> = menu_items.iter()
        .filter(|item| item.is_available)
        .collect();
    
    // Get notices and filter for active ones
    let notices = storage.get_notices()
        .map_err(ApiErrorType::Storage)?;
    let active_notices: Vec<&Notice> = notices.iter()
        .filter(|notice| notice.is_active)
        .collect();
    
    // Prepare context for template
    let mut context = tera::Context::new();
    context.insert("menu_items", &available_menu_items);
    context.insert("notices", &active_notices);
    
    // Render the template
    let rendered = tera.render("menu.html", &context)
        .map_err(|e| ApiErrorType::Validation(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// Menu Schedules Page Handler
pub async fn menu_schedules_page(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    tera: web::Data<Tera>,
) -> Result<HttpResponse, ApiErrorType> {
    println!("DEBUG: menu_schedules_page handler called");
    
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        println!("DEBUG: Authentication failed in menu_schedules_page: {}", e);
        ApiErrorType::Validation(format!("Authentication required: {}", e))
    })?;
    
    // Get menu presets for the dropdown
    let presets = storage.get_menu_presets()
        .map_err(ApiErrorType::Storage)?;
    
    // Get menu schedules
    let schedules = storage.get_menu_schedules()
        .map_err(ApiErrorType::Storage)?;
    
    // Prepare context for template
    let mut context = tera::Context::new();
    context.insert("presets", &presets);
    context.insert("schedules", &schedules);
    
    // Add session data to template context
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("session", &serde_json::json!({
            "username": username,
            "user_id": session.get::<Uuid>("user_id").ok().flatten()
        }));
    }
    
    // Render the template
    let rendered = tera.render("admin/schedules.html", &context)
        .map_err(|e| ApiErrorType::Validation(format!("Template error: {}", e)))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// Menu Presets Page Handler
pub async fn menu_presets_page(
    storage: web::Data<JsonStorage>,
    session: actix_session::Session,
    tera: web::Data<Tera>,
) -> Result<HttpResponse, ApiErrorType> {
    println!("DEBUG: menu_presets_page handler called");
    
    // Check authentication
    let _user_id = require_auth(&session).await.map_err(|e| {
        println!("DEBUG: Authentication failed in menu_presets_page: {}", e);
        ApiErrorType::Validation(format!("Authentication required: {}", e))
    })?;
    
    // Get menu items for the dropdown
    let menu_items = storage.get_menu_items()
        .map_err(ApiErrorType::Storage)?;
    
    // Get menu presets
    let presets = storage.get_menu_presets()
        .map_err(ApiErrorType::Storage)?;
    
    // Prepare context for template
    let mut context = tera::Context::new();
    context.insert("menu_items", &menu_items);
    context.insert("presets", &presets);
    
    // Add session data to template context
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("session", &serde_json::json!({
            "username": username,
            "user_id": session.get::<Uuid>("user_id").ok().flatten()
        }));
    }
    
    // Render the template
    let rendered = tera.render("admin/presets.html", &context)
        .map_err(|e| ApiErrorType::Validation(format!("Template error: {}", e)))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}