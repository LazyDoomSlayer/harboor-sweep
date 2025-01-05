#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::fetch_ports;
pub use windows::get_processes_using_port;
pub use windows::kill_process;
