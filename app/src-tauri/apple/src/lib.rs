//! The App Store's own services for Idlewright (T17): the purchase, the
//! rating prompt, Game Center and iCloud. The page sends one call with a
//! method name and its arguments; on an iPhone it goes to `ApplePlugin.swift`,
//! anywhere else it comes back refused. Like the shim, this decides nothing:
//! `crates/core/tests/boundary.rs` reads it for branches.

use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

#[cfg(target_os = "ios")]
use tauri::Manager;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_apple);

#[cfg(target_os = "ios")]
struct Apple<R: Runtime>(tauri::plugin::PluginHandle<R>);

#[cfg(target_os = "ios")]
#[tauri::command]
async fn call<R: Runtime>(
    app: tauri::AppHandle<R>,
    method: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    app.state::<Apple<R>>()
        .0
        .run_mobile_plugin::<serde_json::Value>(&method, args)
        .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "ios"))]
#[tauri::command]
async fn call(method: String) -> Result<serde_json::Value, String> {
    Err(format!("{method}: only on an iPhone or iPad"))
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("apple")
        .invoke_handler(tauri::generate_handler![call])
        .setup(|app, api| {
            #[cfg(target_os = "ios")]
            let done = api
                .register_ios_plugin(init_plugin_apple)
                .map(|h| {
                    app.manage(Apple(h));
                })
                .map_err(Into::into);
            #[cfg(not(target_os = "ios"))]
            let done = {
                let _ = (app, api);
                Ok(())
            };
            done
        })
        .build()
}
