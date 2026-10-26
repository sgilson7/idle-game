//! Idlewright on an iPhone (T17). The page, the WebAssembly engine and the
//! data are the web build's own files, byte for byte (IG-Q52 A): this crate
//! only does the phone's jobs. It keeps the save in a file, schedules the
//! notifications the core lists, and hands the App Store's services to the
//! Apple plugin. Like the web shim it decides nothing; it translates values
//! and calls the system, and `crates/core/tests/boundary.rs` reads it for
//! branches.

use std::path::PathBuf;

use tauri::{AppHandle, Manager, Runtime};

/// One notification from the core's `alerts` list.
#[derive(serde::Deserialize)]
struct Alert {
    id: i32,
    /// Wall-clock milliseconds.
    at: i64,
    title: String,
    body: String,
}

fn save_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path().app_data_dir().unwrap_or_default().join("save.json")
}

/// The save text, or nothing for a new game.
#[tauri::command]
fn save_read<R: Runtime>(app: AppHandle<R>) -> String {
    std::fs::read_to_string(save_path(&app)).unwrap_or_default()
}

/// Writes the save beside itself first, then moves it into place, so a
/// phone that dies mid-write keeps the last whole save.
#[tauri::command]
fn save_write<R: Runtime>(app: AppHandle<R>, text: String) -> Result<(), String> {
    let path = save_path(&app);
    let next = path.with_extension("json.next");
    path.parent()
        .map(std::fs::create_dir_all)
        .unwrap_or(Ok(()))
        .and_then(|_| std::fs::write(&next, text))
        .and_then(|_| std::fs::rename(&next, &path))
        .map_err(|e| e.to_string())
}

/// Replaces the scheduled notifications with the core's list.
#[cfg(mobile)]
#[tauri::command]
fn alerts_set<R: Runtime>(app: AppHandle<R>, alerts: Vec<Alert>) {
    use tauri_plugin_notification::{NotificationExt, Schedule};
    let n = app.notification();
    let _ = n.request_permission();
    let _ = n.cancel_all();
    alerts.into_iter().for_each(|a| {
        let date = time::OffsetDateTime::from_unix_timestamp_nanos(i128::from(a.at) * 1_000_000)
            .unwrap_or(time::OffsetDateTime::UNIX_EPOCH);
        let _ = n
            .builder()
            .id(a.id)
            .title(a.title)
            .body(a.body)
            .schedule(Schedule::At {
                date,
                repeating: false,
                allow_while_idle: true,
            })
            .show();
    });
}

/// A desktop build is for trying the app out; it schedules nothing.
#[cfg(desktop)]
#[tauri::command]
fn alerts_set(alerts: Vec<Alert>) {
    let _ = alerts.into_iter().map(|a| (a.id, a.at, a.title, a.body)).count();
}

/// Clears the scheduled notifications: the player is back.
#[tauri::command]
fn alerts_clear<R: Runtime>(app: AppHandle<R>) {
    #[cfg(mobile)]
    {
        use tauri_plugin_notification::NotificationExt;
        let _ = app.notification().cancel_all();
    }
    let _ = app;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_apple::init())
        .invoke_handler(tauri::generate_handler![save_read, save_write, alerts_set, alerts_clear])
        .run(tauri::generate_context!())
        .expect("the app could not start")
}
