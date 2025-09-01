use actix_web::web::Data;
use chrono::Utc;
use log::{error, info};
use tokio::time::{Duration, interval};

use crate::storage::{JsonStorage, MenuSchedule, ScheduleRecurrence, ScheduleStatus};

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
        if let Err(e) = check_and_execute_schedules(&storage) {
            error!("Error checking and executing schedules: {}", e);
        }
    }
}

/// Check all schedules and execute any that are due
fn check_and_execute_schedules(
    storage: &Data<JsonStorage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get all schedules
    let schedules = storage.get_menu_schedules()?;

    // Get current time in UTC
    let now = Utc::now();

    // Check each schedule
    for schedule in schedules {
        // Only process schedules that are in Pending status
        if matches!(schedule.status, ScheduleStatus::Pending) {
            // Check if schedule is due
            if is_schedule_due(&schedule, now) {
                info!(
                    "Executing due schedule: {} ({})",
                    schedule.name, schedule.id
                );

                // Execute the schedule
                if let Err(e) = execute_schedule(storage, schedule.clone()) {
                    error!("Failed to execute schedule {}: {}", schedule.id, e);
                    // Update schedule status to Inactive
                    let mut failed_schedule = schedule.clone();
                    failed_schedule.status = ScheduleStatus::Inactive;
                    if let Err(update_err) =
                        storage.update_menu_schedule(schedule.id, failed_schedule)
                    {
                        error!("Failed to update schedule status to Failed: {}", update_err);
                    }
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
fn execute_schedule(
    storage: &Data<JsonStorage>,
    mut schedule: MenuSchedule,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    // Leave is_available status of other items unchanged
    for item in menu_items {
        if preset.menu_item_ids.contains(&item.id) {
            let mut updated_item = item.clone();
            updated_item.is_available = true;
            storage.update_menu_item(updated_item.id, updated_item)?;
        }
    }

    // Update schedule status based on recurrence
    match schedule.recurrence {
        ScheduleRecurrence::Daily | ScheduleRecurrence::Weekly | ScheduleRecurrence::Monthly => {
            // For recurring schedules, calculate next occurrence and keep status as Pending
            if let Some(next_start) = calculate_next_occurrence(&schedule, Utc::now()) {
                schedule.start_time = next_start;
                schedule.updated_at = Utc::now();
                // Keep status as Pending for next execution
            } else {
                // If we can't calculate next occurrence, mark as inactive
                schedule.status = ScheduleStatus::Inactive;
                schedule.updated_at = Utc::now();
            }
        }
        ScheduleRecurrence::Custom => {
            // For custom recurrence, mark as inactive
            schedule.status = ScheduleStatus::Inactive;
            schedule.updated_at = Utc::now();
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
