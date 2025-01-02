use crate::common::{KillProcessResponse, PortInfo, ProcessInfoResponse};
use crate::state::AppState;

use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter, State};

#[cfg(target_family = "unix")]
use crate::unix;

#[tauri::command]
pub fn start_monitoring(app_handle: AppHandle, app_state: State<AppState>) -> Result<(), String> {
    if app_state.is_monitoring() {
        return Err("Monitoring is already running".into());
    }

    app_state.set_monitoring(true);

    let interval = Arc::clone(&app_state.interval);
    let monitoring = Arc::clone(&app_state.is_monitoring);

    let handle = app_handle.clone();
    std::thread::spawn(move || monitor_ports(handle, monitoring, interval));

    Ok(())
}

fn monitor_ports(app_handle: AppHandle, monitoring: Arc<Mutex<bool>>, interval: Arc<Mutex<u64>>) {
    while *monitoring.lock().unwrap() {
        let current_interval = *interval.lock().unwrap();

        match fetch_ports_by_os() {
            Ok(ports) => {
                app_handle.emit("ports_update", &ports).unwrap_or_else(|e| {
                    eprintln!("Failed to emit event: {}", e);
                });
            }
            Err(e) => {
                eprintln!("Error fetching ports: {}", e);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(current_interval));
    }
}

fn fetch_ports_by_os() -> Result<Vec<PortInfo>, String> {
    #[cfg(target_family = "unix")]
    {
        unix::fetch_ports()
    }
}

#[tauri::command]
pub fn stop_monitoring(app_state: State<AppState>) -> Result<(), String> {
    if !app_state.is_monitoring() {
        return Err("Monitoring is not running".to_string());
    }

    app_state.set_monitoring(false);
    Ok(())
}

#[tauri::command]
pub fn update_interval(new_interval: u64, app_state: State<AppState>) -> Result<(), String> {
    if new_interval < 1 || new_interval > 60 {
        return Err("Interval must be between 1 and 60 seconds".to_string());
    }

    app_state.set_interval(new_interval);
    Ok(())
}

#[tauri::command]
pub fn fetch_ports() -> Result<Vec<PortInfo>, String> {
    #[cfg(target_family = "unix")]
    {
        unix::fetch_ports()
    }
}

#[tauri::command]
pub fn kill_process(pid: u32) -> KillProcessResponse {
    #[cfg(target_family = "unix")]
    {
        unix::kill_process(pid)
    }
}

#[tauri::command]
pub fn get_processes_using_port(port: u16, item_pid: u32) -> Result<ProcessInfoResponse, String> {
    #[cfg(target_family = "unix")]
    {
        unix::get_processes_using_port(port, item_pid)
    }
}
