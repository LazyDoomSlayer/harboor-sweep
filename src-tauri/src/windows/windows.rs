use crate::common::{KillProcessResponse, PortInfo, ProcessInfo, ProcessInfoResponse};
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use std::process::Command;

pub fn fetch_ports() -> Result<Vec<PortInfo>, String> {
    let output = Command::new("netstat")
        .args(&["-ano"])
        .output()
        .map_err(|e| format!("Failed to execute netstat: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "netstat command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_netstat_output(&stdout)
}

fn parse_netstat_output(output: &str) -> Result<Vec<PortInfo>, String> {
    let mut seen = HashSet::new();
    let mut ports = Vec::new();

    for line in output.lines().skip(4) {
        // Skip header
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }

        let local_address = parts[1];
        let state = parts[3];
        let pid: u32 = match parts[4].parse() {
            Ok(pid) => pid,
            Err(_) => continue,
        };

        let port: u16 = local_address
            .split(':')
            .last()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);

        let process_path = match get_process_path(pid) {
            Ok(path) => path,
            Err(err) => err,
        };

        let is_listener = state.eq_ignore_ascii_case("LISTENING");

        if seen.insert((pid, port)) {
            ports.push(PortInfo {
                id: generate_unique_id(pid, port, ""),
                pid,
                process_name: get_process_name(pid).unwrap_or_else(|_| "Unknown".to_string()),
                port,
                process_path,
                is_listener,
            });
        }
    }

    Ok(ports)
}

fn generate_unique_id(pid: u32, port: u16, process_name: &str) -> String {
    let mut hasher = DefaultHasher::new();
    pid.hash(&mut hasher);
    port.hash(&mut hasher);
    process_name.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn get_process_path(pid: u32) -> Result<String, String> {
    let output = Command::new("wmic")
        .args(&[
            "process",
            "where",
            &format!("ProcessId={}", pid),
            "get",
            "ExecutablePath",
        ])
        .output()
        .map_err(|e| format!("Failed to execute wmic command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .skip(1)
        .find(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .ok_or_else(|| "Executable path not found".to_string())
}

fn get_process_name(pid: u32) -> Result<String, String> {
    let output = Command::new("tasklist")
        .args(&["/FI", &format!("PID eq {}", pid)])
        .output()
        .map_err(|e| format!("Failed to execute tasklist command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .skip(3)
        .next()
        .and_then(|line| line.split_whitespace().next().map(|s| s.to_string()))
        .ok_or_else(|| "Process name not found".to_string())
}

pub fn kill_process(pid: u32) -> KillProcessResponse {
    let output = Command::new("taskkill")
        .args(&["/PID", &pid.to_string(), "/F"])
        .output();

    match output {
        Ok(output) if output.status.success() => KillProcessResponse {
            success: true,
            message: format!("Successfully killed process with PID {}", pid),
        },
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            KillProcessResponse {
                success: false,
                message: format!("Failed to kill process {}: {}", pid, stderr.trim()),
            }
        }
        Err(e) => KillProcessResponse {
            success: false,
            message: format!("Failed to execute taskkill command: {}", e),
        },
    }
}

pub fn get_processes_using_port(port: u16, item_pid: u32) -> Result<ProcessInfoResponse, String> {
    let processes = fetch_ports()?;
    let target_process = processes
        .iter()
        .find(|p| p.port == port && p.pid != item_pid);

    if let Some(process_info) = target_process {
        Ok(ProcessInfoResponse {
            is_listener: process_info.is_listener,
            data: Some(ProcessInfo {
                pid: process_info.pid,
                port: process_info.port,
                process_name: process_info.process_name.clone(),
                process_path: process_info.process_path.clone(),
            }),
        })
    } else {
        Err(format!("No processes found using port {}", port))
    }
}
