use crate::common::{KillProcessResponse, PortInfo, ProcessInfo};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

fn get_process_path(pid: u32) -> Result<String, String> {
    let exe_path = format!("/proc/{}/exe", pid);
    fs::read_link(&exe_path)
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                "Permission Denied".to_string()
            } else {
                "Unknown".to_string()
            }
        })
}

pub fn fetch_ports() -> Result<Vec<PortInfo>, String> {
    let output = Command::new("lsof")
        .args(["-i", "-P", "-n"])
        .output()
        .map_err(|e| format!("Failed to execute lsof: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "lsof command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_lsof_output(&stdout)
}

fn parse_lsof_output(output: &str) -> Result<Vec<PortInfo>, String> {
    let mut seen = HashSet::new();
    let mut ports = Vec::new();

    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            continue;
        }

        let pid: u32 = match parts[1].parse() {
            Ok(pid) => pid,
            Err(_) => continue,
        };

        let port: u16 = parts[8]
            .split(':')
            .last()
            .unwrap_or("0")
            .parse::<u16>()
            .unwrap_or(0);

        let process_path = match get_process_path(pid) {
            Ok(path) => path,
            Err(err) => err,
        };

        if seen.insert((pid, port.clone())) {
            ports.push(PortInfo {
                id: Uuid::new_v4().to_string(),
                pid,
                process_name: parts[0].to_string(),
                port,
                process_path,
            });
        }
    }

    Ok(ports)
}

pub fn kill_process(pid: u32) -> KillProcessResponse {
    let output = Command::new("kill").arg(pid.to_string()).output();

    match output {
        Ok(output) if output.status.success() => KillProcessResponse {
            success: true,
            message: format!("Successfully killed process with PID {}", pid),
        },
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let exit_code = output.status.code().unwrap_or(-1);
            KillProcessResponse {
                success: false,
                message: format!(
                    "Failed to kill process {} (Exit code: {}): {}",
                    pid,
                    exit_code,
                    stderr.trim()
                ),
            }
        }
        Err(e) => KillProcessResponse {
            success: false,
            message: format!("Failed to execute kill command: {}", e),
        },
    }
}

pub fn get_processes_using_port(port: u16) -> Result<Vec<ProcessInfo>, String> {
    let output = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{}", port))
        .output()
        .map_err(|e| format!("Failed to execute lsof command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "lsof command failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut processes = vec![];

    for line in stdout.lines() {
        if line.contains("LISTEN") {
            let fields: Vec<&str> = line.split_whitespace().collect();

            if fields.len() < 9 {
                continue;
            }

            let pid: u32 = match fields[1].parse() {
                Ok(pid) => pid,
                Err(_) => continue,
            };

            if let Some(process_info) = get_process_info(pid, port) {
                processes.push(process_info);
            }
        }
    }

    if processes.is_empty() {
        return Err(format!("No processes found listening on port {}", port));
    }

    Ok(processes)
}

fn get_process_info(pid: u32, port: u16) -> Option<ProcessInfo> {
    let proc_path = PathBuf::from(format!("/proc/{}/", pid));

    let name = fs::read_to_string(proc_path.join("comm"))
        .ok()?
        .trim()
        .to_string();

    let path = fs::read_link(proc_path.join("exe"))
        .ok()?
        .to_string_lossy()
        .to_string();

    let status = fs::read_to_string(proc_path.join("status")).ok()?;
    let parent_pid = status
        .lines()
        .find(|line| line.starts_with("PPid:"))?
        .split_whitespace()
        .nth(1)?
        .parse::<u32>()
        .ok();

    Some(ProcessInfo {
        pid,
        port,
        name,
        path,
        parent_pid,
    })
}
