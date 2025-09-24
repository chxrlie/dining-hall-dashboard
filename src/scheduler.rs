use actix_web::web::Data;
use chrono::Utc;
use log::{error, info, warn};
use tokio::time::{Duration, interval};

use crate::storage::{JsonStorage, MenuSchedule, ScheduleRecurrence, ScheduleStatus};

/// Check if a schedule conflicts with any existing schedules
/// A conflict occurs if the time ranges overlap
pub fn has_schedule_conflict(
    schedule: &MenuSchedule,
    existing_schedules: &[MenuSchedule],
) -> Option<MenuSchedule> {
    for existing in existing_schedules {
        // Skip the schedule itself if updating
        if existing.id == schedule.id {
            continue;
        }

        // Check for time overlap
        if schedule.start_time <= existing.end_time && schedule.end_time >= existing.start_time {
            return Some(existing.clone());
        }
    }
    None
}

/// Starts the scheduler service that runs in the background
/// checking for due menu schedules and executing them
pub async fn start_scheduler(storage: Data<JsonStorage>) {
    info!("Starting scheduler service");

    // Spawn the scheduler task as a background process
    tokio::spawn(async move {
        run_scheduler(storage).await;
    });
}

/// Main scheduler loop that runs every minute
async fn run_scheduler(storage: Data<JsonStorage>) {
    // Check every minute
    let mut interval = interval(Duration::from_secs(60));

    // Skip the first immediate tick to align with minute boundaries
    interval.tick().await;

    loop {
        interval.tick().await;
        info!("Scheduler tick: checking for due schedules");

        // Check and execute due schedules
        if let Err(e) = check_and_execute_schedules(&storage).await {
            error!("Error checking and executing schedules: {}", e);
        }
    }
}

/// Check all schedules and execute any that are due
async fn check_and_execute_schedules(
    storage: &Data<JsonStorage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get all schedules
    let schedules = storage.get_menu_schedules()?;

    // Get current time in UTC
    let now = Utc::now();

    // Check each schedule
    for schedule in &schedules {
        // Process schedules that are in Pending status
        if matches!(schedule.status, ScheduleStatus::Pending) {
            // Check if schedule is due
            if is_schedule_due(schedule, now) {
                // Check for conflicts before executing
                if let Some(conflicting_schedule) = has_schedule_conflict(schedule, &schedules) {
                    warn!(
                        "Schedule {} ({}) conflicts with {} ({}), skipping execution",
                        schedule.name,
                        schedule.id,
                        conflicting_schedule.name,
                        conflicting_schedule.id
                    );
                    // Update schedule status to Conflicted
                    let mut conflicted_schedule = schedule.clone();
                    conflicted_schedule.status = ScheduleStatus::Conflicted;
                    conflicted_schedule.error_message = Some(format!(
                        "Conflicts with schedule '{}' ({})",
                        conflicting_schedule.name, conflicting_schedule.id
                    ));
                    if let Err(update_err) =
                        storage.update_menu_schedule(schedule.id, conflicted_schedule)
                    {
                        error!(
                            "Failed to update schedule status to Conflicted: {}",
                            update_err
                        );
                    }
                    continue;
                }

                info!(
                    "Executing due schedule: {} ({})",
                    schedule.name, schedule.id
                );

                // Execute the schedule
                if let Err(e) = execute_schedule(storage, schedule.clone()).await {
                    error!("Failed to execute schedule {}: {}", schedule.id, e);
                    // Update schedule status to Failed
                    let mut failed_schedule = schedule.clone();
                    failed_schedule.status = ScheduleStatus::Failed;
                    failed_schedule.error_message = Some(e.to_string());
                    if let Err(update_err) =
                        storage.update_menu_schedule(schedule.id, failed_schedule)
                    {
                        error!("Failed to update schedule status to Failed: {}", update_err);
                    }
                }
            }
        } else if matches!(schedule.status, ScheduleStatus::Active) {
            // Check if Active schedule has ended
            if schedule.end_time <= now {
                info!(
                    "Active schedule {} has ended, setting to Ended",
                    schedule.id
                );
                let mut ended_schedule = schedule.clone();
                ended_schedule.status = ScheduleStatus::Ended;
                ended_schedule.updated_at = now;
                ended_schedule.error_message = None;
                if let Err(update_err) = storage.update_menu_schedule(schedule.id, ended_schedule) {
                    error!("Failed to update active schedule to Ended: {}", update_err);
                }
            }
        }
    }

    Ok(())
}

/// Check if a schedule is due to run
fn is_schedule_due(schedule: &MenuSchedule, now: chrono::DateTime<Utc>) -> bool {
    schedule.start_time <= now
}

/// Execute a schedule by updating menu items based on the associated preset
async fn execute_schedule(
    storage: &Data<JsonStorage>,
    mut schedule: MenuSchedule,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set status to Active during execution
    schedule.status = ScheduleStatus::Active;
    schedule.updated_at = Utc::now();
    storage.update_menu_schedule(schedule.id, schedule.clone())?;

    // Get the associated preset
    let presets = storage.get_menu_presets()?;
    let preset = presets
        .into_iter()
        .find(|p| p.id == schedule.preset_id)
        .ok_or_else(|| {
            format!(
                "Preset with id {} not found for schedule {}",
                schedule.preset_id, schedule.id
            )
        })?;

    // Get all menu items
    let menu_items = storage.get_menu_items()?;

    // Update menu items based on the preset
    // Set is_available = true for items in the preset
    // Set is_available = false for items not in the preset
    for mut item in menu_items {
        if preset.menu_item_ids.contains(&item.id) {
            item.is_available = true;
        } else {
            item.is_available = false;
        }
        storage.update_menu_item(item.id, item)?;
    }

    // Update schedule status based on recurrence and end time
    let now = Utc::now();

    // Check if schedule has ended
    if schedule.end_time <= now {
        // Schedule has ended, mark as ended
        schedule.status = ScheduleStatus::Ended;
        schedule.updated_at = now;
        schedule.error_message = None;
    } else {
        // Schedule is still active, update based on recurrence
        match schedule.recurrence {
            ScheduleRecurrence::Daily
            | ScheduleRecurrence::Weekly
            | ScheduleRecurrence::Monthly => {
                // For recurring schedules, calculate next occurrence and set status to Pending
                if let Some(next_start) = calculate_next_occurrence(&schedule, now) {
                    // Check if next occurrence is before or at end time
                    if next_start <= schedule.end_time {
                        schedule.start_time = next_start;
                        schedule.status = ScheduleStatus::Pending;
                        schedule.updated_at = now;
                        schedule.error_message = None; // Clear any previous error
                    } else {
                        // Next occurrence would be after end time, mark as ended
                        schedule.status = ScheduleStatus::Ended;
                        schedule.updated_at = now;
                        schedule.error_message =
                            Some("Next occurrence is after schedule end time".to_string());
                    }
                } else {
                    // If we can't calculate next occurrence, mark as ended
                    schedule.status = ScheduleStatus::Ended;
                    schedule.updated_at = now;
                    schedule.error_message = Some("Cannot calculate next occurrence".to_string());
                }
            }
            ScheduleRecurrence::Custom => {
                // For custom recurrence, mark as ended after execution
                schedule.status = ScheduleStatus::Ended;
                schedule.updated_at = now;
                schedule.error_message = None;
            }
        }
    }

    // Update the schedule in storage
    storage.update_menu_schedule(schedule.id, schedule.clone())?;

    info!(
        "Successfully executed schedule: {} ({})",
        schedule.name, schedule.id
    );
    Ok(())
}

/// Calculate the next occurrence of a recurring schedule
fn calculate_next_occurrence(
    schedule: &MenuSchedule,
    _now: chrono::DateTime<Utc>,
) -> Option<chrono::DateTime<Utc>> {
    match schedule.recurrence {
        ScheduleRecurrence::Daily => {
            // Add one day
            Some(schedule.start_time + chrono::Duration::days(1))
        }
        ScheduleRecurrence::Weekly => {
            // Add one week
            Some(schedule.start_time + chrono::Duration::weeks(1))
        }
        ScheduleRecurrence::Monthly => {
            // For monthly, we add one month
            if let Some(next_month) = schedule
                .start_time
                .date_naive()
                .checked_add_months(chrono::Months::new(1))
            {
                Some(next_month.and_time(schedule.start_time.time()).and_utc())
            } else {
                None
            }
        }
        ScheduleRecurrence::Custom => None, // Custom recurrence not implemented yet
    }
}
