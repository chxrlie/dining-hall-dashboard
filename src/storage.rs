use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::error_handler::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuItem {
    pub id: Uuid,
    pub name: String,
    pub category: MenuCategory,
    pub description: String,
    pub allergens: Vec<String>,
    pub is_available: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MenuCategory {
    Mains,
    Sides,
    Desserts,
    Beverages,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notice {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScheduleRecurrence {
    Daily,
    Weekly,
    Monthly,
    Custom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScheduleStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuPreset {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub menu_item_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuSchedule {
    pub id: Uuid,
    pub preset_id: Uuid,
    pub name: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub recurrence: ScheduleRecurrence,
    pub status: ScheduleStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Mutex poison error")]
    PoisonError,
}

impl From<StorageError> for AppError {
    fn from(storage_error: StorageError) -> Self {
        match storage_error {
            StorageError::Io(io_error) => AppError::Storage(io_error.to_string()),
            StorageError::Json(json_error) => AppError::Storage(json_error.to_string()),
            StorageError::PoisonError => AppError::Storage("Poison error".to_string()),
        }
    }
}

pub struct JsonStorage {
    menu_items: Arc<Mutex<Vec<MenuItem>>>,
    notices: Arc<Mutex<Vec<Notice>>>,
    admin_users: Arc<Mutex<Vec<AdminUser>>>,
    menu_presets: Arc<Mutex<Vec<MenuPreset>>>,
    menu_schedules: Arc<Mutex<Vec<MenuSchedule>>>,
    menu_items_path: String,
    notices_path: String,
    admin_users_path: String,
    menu_presets_path: String,
    menu_schedules_path: String,
}

impl JsonStorage {
    pub fn new(
        menu_items_path: &str,
        notices_path: &str,
        admin_users_path: &str,
        menu_presets_path: &str,
        menu_schedules_path: &str,
    ) -> Result<Self, StorageError> {
        log::debug!("JsonStorage::new() started");

        // Ensure data directory exists
        let data_dir = Path::new(menu_items_path)
            .parent()
            .unwrap_or(Path::new("."));
        if !data_dir.exists() {
            log::debug!("Creating data directory: {:?}", data_dir);
            fs::create_dir_all(data_dir)?;
        }

        // Initialize with empty vectors
        let menu_items = Arc::new(Mutex::new(Vec::new()));
        let notices = Arc::new(Mutex::new(Vec::new()));
        let admin_users = Arc::new(Mutex::new(Vec::new()));
        let menu_presets = Arc::new(Mutex::new(Vec::new()));
        let menu_schedules = Arc::new(Mutex::new(Vec::new()));

        let storage = Self {
            menu_items,
            notices,
            admin_users,
            menu_presets,
            menu_schedules,
            menu_items_path: menu_items_path.to_string(),
            notices_path: notices_path.to_string(),
            admin_users_path: admin_users_path.to_string(),
            menu_presets_path: menu_presets_path.to_string(),
            menu_schedules_path: menu_schedules_path.to_string(),
        };

        // Load existing data or create empty files
        log::debug!("Loading menu items...");
        storage.load_menu_items()?;
        log::debug!("Menu items loaded successfully");

        log::debug!("Loading notices...");
        storage.load_notices()?;
        log::debug!("Notices loaded successfully");

        log::debug!("Loading admin users...");
        storage.load_admin_users()?;
        log::debug!("Admin users loaded successfully");

        log::debug!("Loading menu presets...");
        storage.load_menu_presets()?;
        log::debug!("Menu presets loaded successfully");

        log::debug!("Loading menu schedules...");
        storage.load_menu_schedules()?;
        log::debug!("Menu schedules loaded successfully");

        log::debug!("JsonStorage::new() completed");
        Ok(storage)
    }

    pub fn load_menu_items(&self) -> Result<(), StorageError> {
        log::debug!(
            "load_menu_items() started for path: {}",
            self.menu_items_path
        );
        let path = Path::new(&self.menu_items_path);
        if !path.exists() {
            log::debug!("Creating empty menu items file");
            // Create empty file with empty array
            let empty_vec: Vec<MenuItem> = Vec::new();
            let json_data = serde_json::to_string_pretty(&empty_vec)?;
            fs::write(path, json_data)?;
        }

        log::debug!("Reading menu items file");
        let file_content = fs::read_to_string(path)?;
        let items: Vec<MenuItem> = serde_json::from_str(&file_content)?;

        log::debug!("Acquiring menu items mutex");
        let mut menu_items = self
            .menu_items
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        *menu_items = items;
        log::debug!("Menu items loaded: {} items", menu_items.len());

        Ok(())
    }

    pub fn save_menu_items(&self) -> Result<(), StorageError> {
        let menu_items = self
            .menu_items
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        let json_data = serde_json::to_string_pretty(&*menu_items)?;
        fs::write(&self.menu_items_path, json_data)?;
        Ok(())
    }

    pub fn load_notices(&self) -> Result<(), StorageError> {
        log::debug!("load_notices() started for path: {}", self.notices_path);
        let path = Path::new(&self.notices_path);
        if !path.exists() {
            log::debug!("Creating empty notices file");
            // Create empty file with empty array
            let empty_vec: Vec<Notice> = Vec::new();
            let json_data = serde_json::to_string_pretty(&empty_vec)?;
            fs::write(path, json_data)?;
        }

        log::debug!("Reading notices file");
        let file_content = fs::read_to_string(path)?;
        let notices: Vec<Notice> = serde_json::from_str(&file_content)?;

        log::debug!("Acquiring notices mutex");
        let mut notices_lock = self.notices.lock().map_err(|_| StorageError::PoisonError)?;
        *notices_lock = notices;
        log::debug!("Notices loaded: {} items", notices_lock.len());

        Ok(())
    }

    pub fn load_admin_users(&self) -> Result<(), StorageError> {
        log::debug!(
            "load_admin_users() started for path: {}",
            self.admin_users_path
        );
        let path = Path::new(&self.admin_users_path);
        if !path.exists() {
            log::debug!("Creating empty admin users file");
            // Create empty file with empty array
            let empty_vec: Vec<AdminUser> = Vec::new();
            let json_data = serde_json::to_string_pretty(&empty_vec)?;
            fs::write(path, json_data)?;
        }

        log::debug!("Reading admin users file");
        let file_content = fs::read_to_string(path)?;
        let users: Vec<AdminUser> = serde_json::from_str(&file_content)?;

        log::debug!("Acquiring admin users mutex");
        let mut admin_users_lock = self
            .admin_users
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        *admin_users_lock = users;
        log::debug!("Admin users loaded: {} users", admin_users_lock.len());

        Ok(())
    }

    pub fn save_notices(&self) -> Result<(), StorageError> {
        let notices = self.notices.lock().map_err(|_| StorageError::PoisonError)?;
        let json_data = serde_json::to_string_pretty(&*notices)?;
        fs::write(&self.notices_path, json_data)?;
        Ok(())
    }

    pub fn save_admin_users(&self) -> Result<(), StorageError> {
        log::debug!("save_admin_users() started");
        let admin_users = self
            .admin_users
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        log::debug!("Admin users mutex acquired for saving");
        let json_data = serde_json::to_string_pretty(&*admin_users)?;
        log::debug!("JSON serialization completed");
        fs::write(&self.admin_users_path, json_data)?;
        log::debug!("File write completed");
        Ok(())
    }

    pub fn load_menu_presets(&self) -> Result<(), StorageError> {
        log::debug!(
            "load_menu_presets() started for path: {}",
            self.menu_presets_path
        );
        let path = Path::new(&self.menu_presets_path);
        if !path.exists() {
            log::debug!("Creating empty menu presets file");
            // Create empty file with empty array
            let empty_vec: Vec<MenuPreset> = Vec::new();
            let json_data = serde_json::to_string_pretty(&empty_vec)?;
            fs::write(path, json_data)?;
        }

        log::debug!("Reading menu presets file");
        let file_content = fs::read_to_string(path)?;
        let presets: Vec<MenuPreset> = serde_json::from_str(&file_content)?;

        log::debug!("Acquiring menu presets mutex");
        let mut menu_presets = self
            .menu_presets
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        *menu_presets = presets;
        log::debug!("Menu presets loaded: {} items", menu_presets.len());

        Ok(())
    }

    pub fn load_menu_schedules(&self) -> Result<(), StorageError> {
        log::debug!(
            "load_menu_schedules() started for path: {}",
            self.menu_schedules_path
        );
        let path = Path::new(&self.menu_schedules_path);
        if !path.exists() {
            log::debug!("Creating empty menu schedules file");
            // Create empty file with empty array
            let empty_vec: Vec<MenuSchedule> = Vec::new();
            let json_data = serde_json::to_string_pretty(&empty_vec)?;
            fs::write(path, json_data)?;
        }

        log::debug!("Reading menu schedules file");
        let file_content = fs::read_to_string(path)?;
        let schedules: Vec<MenuSchedule> = serde_json::from_str(&file_content)?;

        log::debug!("Acquiring menu schedules mutex");
        let mut menu_schedules = self
            .menu_schedules
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        *menu_schedules = schedules;
        log::debug!("Menu schedules loaded: {} items", menu_schedules.len());

        Ok(())
    }

    pub fn save_menu_presets(&self) -> Result<(), StorageError> {
        let menu_presets = self
            .menu_presets
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        let json_data = serde_json::to_string_pretty(&*menu_presets)?;
        fs::write(&self.menu_presets_path, json_data)?;
        Ok(())
    }

    pub fn save_menu_schedules(&self) -> Result<(), StorageError> {
        let menu_schedules = self
            .menu_schedules
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        let json_data = serde_json::to_string_pretty(&*menu_schedules)?;
        fs::write(&self.menu_schedules_path, json_data)?;
        Ok(())
    }

    pub fn get_menu_items(&self) -> Result<Vec<MenuItem>, StorageError> {
        let menu_items = self
            .menu_items
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        Ok(menu_items.clone())
    }

    pub fn get_notices(&self) -> Result<Vec<Notice>, StorageError> {
        let notices = self.notices.lock().map_err(|_| StorageError::PoisonError)?;
        Ok(notices.clone())
    }

    pub fn get_admin_users(&self) -> Result<Vec<AdminUser>, StorageError> {
        let admin_users = self
            .admin_users
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        Ok(admin_users.clone())
    }

    pub fn add_menu_item(&self, item: MenuItem) -> Result<(), AppError> {
        let mut menu_items = self
            .menu_items
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        menu_items.push(item);
        // Explicitly drop the lock before calling save_menu_items
        drop(menu_items);
        self.save_menu_items().map_err(|e| AppError::from(e))
    }

    pub fn add_notice(&self, notice: Notice) -> Result<(), StorageError> {
        let mut notices = self.notices.lock().map_err(|_| StorageError::PoisonError)?;
        notices.push(notice);
        // Explicitly drop the lock before calling save_notices
        drop(notices);
        self.save_notices()
    }

    pub fn update_menu_item(&self, id: Uuid, updated_item: MenuItem) -> Result<(), AppError> {
        log::debug!(
            "update_menu_item() called with id: {}, item: {:?}",
            id,
            updated_item
        );
        log::debug!("About to acquire menu_items lock in update_menu_item");
        let mut menu_items = self
            .menu_items
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        log::debug!("Acquired menu_items lock in update_menu_item");
        if let Some(index) = menu_items.iter().position(|item| item.id == id) {
            menu_items[index] = updated_item;
            log::debug!("Item updated in memory");
            // Explicitly drop the lock before calling save_menu_items
            drop(menu_items);
            log::debug!("Released menu_items lock in update_menu_item");
            log::debug!("About to call save_menu_items()");
            self.save_menu_items().map_err(|e| AppError::from(e))?;
            log::debug!("save_menu_items() completed successfully");
            Ok(())
        } else {
            // Explicitly drop the lock before returning error
            drop(menu_items);
            log::debug!("Released menu_items lock in update_menu_item (not found)");
            Err(AppError::Storage(format!(
                "Menu item with id {} not found",
                id
            )))
        }
    }

    pub fn delete_menu_item(&self, id: Uuid) -> Result<(), AppError> {
        log::debug!("delete_menu_item() called with id: {}", id);
        log::debug!("About to acquire menu_items lock in delete_menu_item");
        let mut menu_items = self
            .menu_items
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        log::debug!("Acquired menu_items lock in delete_menu_item");
        if let Some(index) = menu_items.iter().position(|item| item.id == id) {
            menu_items.remove(index);
            log::debug!("Item removed from memory");
            // Explicitly drop the lock before calling save_menu_items
            drop(menu_items);
            log::debug!("Released menu_items lock in delete_menu_item");
            log::debug!("About to call save_menu_items()");
            self.save_menu_items().map_err(|e| AppError::from(e))?;
            log::debug!("save_menu_items() completed successfully");
            Ok(())
        } else {
            // Explicitly drop the lock before returning error
            drop(menu_items);
            log::debug!("Released menu_items lock in delete_menu_item (not found)");
            Err(AppError::Storage(format!(
                "Menu item with id {} not found",
                id
            )))
        }
    }

    pub fn update_notice(&self, id: Uuid, updated_notice: Notice) -> Result<(), StorageError> {
        log::debug!(
            "update_notice() called with id: {}, notice: {:?}",
            id,
            updated_notice
        );
        log::debug!("About to acquire notices lock in update_notice");
        let mut notices = self.notices.lock().map_err(|_| StorageError::PoisonError)?;
        log::debug!("Acquired notices lock in update_notice");
        if let Some(index) = notices.iter().position(|notice| notice.id == id) {
            notices[index] = updated_notice;
            log::debug!("Notice updated in memory");
            // Explicitly drop the lock before calling save_notices
            drop(notices);
            log::debug!("Released notices lock in update_notice");
            log::debug!("About to call save_notices()");
            self.save_notices()?;
            log::debug!("save_notices() completed successfully");
            Ok(())
        } else {
            // Explicitly drop the lock before returning error
            drop(notices);
            log::debug!("Released notices lock in update_notice (not found)");
            Err(StorageError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Notice with id {} not found", id),
            )))
        }
    }

    pub fn delete_notice(&self, id: Uuid) -> Result<(), StorageError> {
        log::debug!("delete_notice() called with id: {}", id);
        log::debug!("About to acquire notices lock in delete_notice");
        let mut notices = self.notices.lock().map_err(|_| StorageError::PoisonError)?;
        log::debug!("Acquired notices lock in delete_notice");
        if let Some(index) = notices.iter().position(|notice| notice.id == id) {
            notices.remove(index);
            log::debug!("Notice removed from memory");
            // Explicitly drop the lock before calling save_notices
            drop(notices);
            log::debug!("Released notices lock in delete_notice");
            log::debug!("About to call save_notices()");
            self.save_notices()?;
            log::debug!("save_notices() completed successfully");
            Ok(())
        } else {
            // Explicitly drop the lock before returning error
            drop(notices);
            log::debug!("Released notices lock in delete_notice (not found)");
            Err(StorageError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Notice with id {} not found", id),
            )))
        }
    }

    pub fn get_admin_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<AdminUser>, StorageError> {
        let admin_users = self
            .admin_users
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        let user = admin_users
            .iter()
            .find(|user| user.username == username)
            .cloned();
        Ok(user)
    }

    pub fn add_admin_user(&self, user: AdminUser) -> Result<(), StorageError> {
        log::debug!("add_admin_user() started");
        {
            let mut admin_users = self
                .admin_users
                .lock()
                .map_err(|_| StorageError::PoisonError)?;
            log::debug!("Admin users mutex acquired");
            admin_users.push(user);
            log::debug!("User added to vector");
        } // Lock is released here
        log::debug!("Saving admin users...");
        self.save_admin_users()?;
        log::debug!("add_admin_user() completed successfully");
        Ok(())
    }

    pub fn get_menu_presets(&self) -> Result<Vec<MenuPreset>, StorageError> {
        let menu_presets = self
            .menu_presets
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        Ok(menu_presets.clone())
    }

    pub fn get_menu_schedules(&self) -> Result<Vec<MenuSchedule>, StorageError> {
        let menu_schedules = self
            .menu_schedules
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        Ok(menu_schedules.clone())
    }

    pub fn add_menu_preset(&self, preset: MenuPreset) -> Result<(), StorageError> {
        let mut menu_presets = self
            .menu_presets
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        menu_presets.push(preset);
        // Explicitly drop the lock before calling save_menu_presets
        drop(menu_presets);
        self.save_menu_presets()
    }

    pub fn add_menu_schedule(&self, schedule: MenuSchedule) -> Result<(), StorageError> {
        let mut menu_schedules = self
            .menu_schedules
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        menu_schedules.push(schedule);
        // Explicitly drop the lock before calling save_menu_schedules
        drop(menu_schedules);
        self.save_menu_schedules()
    }

    pub fn update_menu_preset(
        &self,
        id: Uuid,
        updated_preset: MenuPreset,
    ) -> Result<(), StorageError> {
        log::debug!(
            "update_menu_preset() called with id: {}, preset: {:?}",
            id,
            updated_preset
        );
        log::debug!("About to acquire menu_presets lock in update_menu_preset");
        let mut menu_presets = self
            .menu_presets
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        log::debug!("Acquired menu_presets lock in update_menu_preset");
        if let Some(index) = menu_presets.iter().position(|preset| preset.id == id) {
            menu_presets[index] = updated_preset;
            log::debug!("Preset updated in memory");
            // Explicitly drop the lock before calling save_menu_presets
            drop(menu_presets);
            log::debug!("Released menu_presets lock in update_menu_preset");
            log::debug!("About to call save_menu_presets()");
            self.save_menu_presets()?;
            log::debug!("save_menu_presets() completed successfully");
            Ok(())
        } else {
            // Explicitly drop the lock before returning error
            drop(menu_presets);
            log::debug!("Released menu_presets lock in update_menu_preset (not found)");
            Err(StorageError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Menu preset with id {} not found", id),
            )))
        }
    }

    pub fn update_menu_schedule(
        &self,
        id: Uuid,
        updated_schedule: MenuSchedule,
    ) -> Result<(), StorageError> {
        log::debug!(
            "update_menu_schedule() called with id: {}, schedule: {:?}",
            id,
            updated_schedule
        );
        log::debug!("About to acquire menu_schedules lock in update_menu_schedule");
        let mut menu_schedules = self
            .menu_schedules
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        log::debug!("Acquired menu_schedules lock in update_menu_schedule");
        if let Some(index) = menu_schedules.iter().position(|schedule| schedule.id == id) {
            menu_schedules[index] = updated_schedule;
            log::debug!("Schedule updated in memory");
            // Explicitly drop the lock before calling save_menu_schedules
            drop(menu_schedules);
            log::debug!("Released menu_schedules lock in update_menu_schedule");
            log::debug!("About to call save_menu_schedules()");
            self.save_menu_schedules()?;
            log::debug!("save_menu_schedules() completed successfully");
            Ok(())
        } else {
            // Explicitly drop the lock before returning error
            drop(menu_schedules);
            log::debug!("Released menu_schedules lock in update_menu_schedule (not found)");
            Err(StorageError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Menu schedule with id {} not found", id),
            )))
        }
    }

    pub fn delete_menu_preset(&self, id: Uuid) -> Result<(), StorageError> {
        log::debug!("delete_menu_preset() called with id: {}", id);
        log::debug!("About to acquire menu_presets lock in delete_menu_preset");
        let mut menu_presets = self
            .menu_presets
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        log::debug!("Acquired menu_presets lock in delete_menu_preset");
        if let Some(index) = menu_presets.iter().position(|preset| preset.id == id) {
            menu_presets.remove(index);
            log::debug!("Preset removed from memory");
            // Explicitly drop the lock before calling save_menu_presets
            drop(menu_presets);
            log::debug!("Released menu_presets lock in delete_menu_preset");
            log::debug!("About to call save_menu_presets()");
            self.save_menu_presets()?;
            log::debug!("save_menu_presets() completed successfully");
            Ok(())
        } else {
            // Explicitly drop the lock before returning error
            drop(menu_presets);
            log::debug!("Released menu_presets lock in delete_menu_preset (not found)");
            Err(StorageError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Menu preset with id {} not found", id),
            )))
        }
    }

    pub fn delete_menu_schedule(&self, id: Uuid) -> Result<(), StorageError> {
        log::debug!("delete_menu_schedule() called with id: {}", id);
        log::debug!("About to acquire menu_schedules lock in delete_menu_schedule");
        let mut menu_schedules = self
            .menu_schedules
            .lock()
            .map_err(|_| StorageError::PoisonError)?;
        log::debug!("Acquired menu_schedules lock in delete_menu_schedule");
        if let Some(index) = menu_schedules.iter().position(|schedule| schedule.id == id) {
            menu_schedules.remove(index);
            log::debug!("Schedule removed from memory");
            // Explicitly drop the lock before calling save_menu_schedules
            drop(menu_schedules);
            log::debug!("Released menu_schedules lock in delete_menu_schedule");
            log::debug!("About to call save_menu_schedules()");
            self.save_menu_schedules()?;
            log::debug!("save_menu_schedules() completed successfully");
            Ok(())
        } else {
            // Explicitly drop the lock before returning error
            drop(menu_schedules);
            log::debug!("Released menu_schedules lock in delete_menu_schedule (not found)");
            Err(StorageError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Menu schedule with id {} not found", id),
            )))
        }
    }
}
