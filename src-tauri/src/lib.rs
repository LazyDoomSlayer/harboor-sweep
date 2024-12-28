// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod common;
use crate::common::PortInfo;

#[cfg(target_family = "unix")]
mod unix;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn fetch_ports() -> Result<Vec<PortInfo>, String> {
    #[cfg(target_family = "unix")]
    {
        unix::fetch_ports()
    }
}
