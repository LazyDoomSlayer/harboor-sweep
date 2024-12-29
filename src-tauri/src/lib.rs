mod commands;
mod common;
mod state;

use state::AppState;

use commands::{fetch_ports, kill_process, start_monitoring, stop_monitoring, update_interval};

#[cfg(target_family = "unix")]
pub mod unix;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new(5);

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_ports,
            kill_process,
            start_monitoring,
            stop_monitoring,
            update_interval
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
