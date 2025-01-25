use windows::Win32::Foundation::NO_ERROR;
use windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, GetExtendedUdpTable, TCP_TABLE_OWNER_PID_ALL, UDP_TABLE_OWNER_PID,
    MIB_TCPTABLE_OWNER_PID, MIB_TCP6TABLE_OWNER_PID, MIB_UDPTABLE_OWNER_PID, MIB_UDP6TABLE_OWNER_PID
};
use windows::Win32::System::Threading::{OpenProcess, TerminateProcess};
use windows::Win32::Foundation::{CloseHandle, ERROR_ACCESS_DENIED};
use windows::Win32::System::Threading::PROCESS_TERMINATE;

use crate::common::{KillProcessResponse, PortInfo};


// TCP State Constants
const TCP_STATE_LISTEN: u32 = 2;

/// Enum to represent the protocol and address family
#[derive(Debug)]
enum Protocol {
    TcpIpv4,
    TcpIpv6,
    UdpIpv4,
    UdpIpv6,
}

/// Function to determine the required buffer size for TCP/UDP tables
fn get_buffer_size(protocol: &Protocol) -> Option<u32> {
    let mut buffer_size = 0u32;

    unsafe {
        let result = match protocol {
            Protocol::TcpIpv4 => GetExtendedTcpTable(None, &mut buffer_size, false, 2, TCP_TABLE_OWNER_PID_ALL, 0),
            Protocol::TcpIpv6 => GetExtendedTcpTable(None, &mut buffer_size, false, 23, TCP_TABLE_OWNER_PID_ALL, 0),
            Protocol::UdpIpv4 => GetExtendedUdpTable(None, &mut buffer_size, false, 2, UDP_TABLE_OWNER_PID, 0),
            Protocol::UdpIpv6 => GetExtendedUdpTable(None, &mut buffer_size, false, 23, UDP_TABLE_OWNER_PID, 0),
        };

        if result == 122 { // ERROR_INSUFFICIENT_BUFFER
            Some(buffer_size)
        } else {
            println!("Unexpected result during first call: {}", result);
            None
        }
    }
}

/// Function to retrieve the TCP/UDP table
fn fetch_table(protocol: &Protocol, buffer_size: u32) -> Option<Vec<u8>> {
    // Allocate the buffer with the required size
    let mut buffer = vec![0u8; buffer_size as usize];

    unsafe {
        let result = match protocol {
            Protocol::TcpIpv4 => GetExtendedTcpTable(Some(buffer.as_mut_ptr() as *mut _), &mut (buffer_size as u32), false, 2, TCP_TABLE_OWNER_PID_ALL, 0),
            Protocol::TcpIpv6 => GetExtendedTcpTable(Some(buffer.as_mut_ptr() as *mut _), &mut (buffer_size as u32), false, 23, TCP_TABLE_OWNER_PID_ALL, 0),
            Protocol::UdpIpv4 => GetExtendedUdpTable(Some(buffer.as_mut_ptr() as *mut _), &mut (buffer_size as u32), false, 2, UDP_TABLE_OWNER_PID, 0),
            Protocol::UdpIpv6 => GetExtendedUdpTable(Some(buffer.as_mut_ptr() as *mut _), &mut (buffer_size as u32), false, 23, UDP_TABLE_OWNER_PID, 0),
        };

        if result == NO_ERROR.0 {
            println!("Successfully retrieved the table for protocol: {:?}", protocol);
            Some(buffer) // Return the buffer containing the table data
        } else {
            println!("Failed to retrieve table for protocol: {:?}. Error code: {}", protocol, result);
            None
        }
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a unique ID by hashing the pid and port
fn generate_unique_id(pid: u32, port: u16) -> String {
    let mut hasher = DefaultHasher::new();
    pid.hash(&mut hasher);
    port.hash(&mut hasher);
    format!("{:x}", hasher.finish()) // Convert the hash to a hexadecimal string
}



/// Parse the TCP/IPv4 table buffer into a list of PortInfo
fn parse_tcp_ipv4(buffer: &[u8]) -> Vec<PortInfo> {
    let mut results = Vec::new();

    unsafe {
        // Interpret the buffer as a pointer to MIB_TCPTABLE_OWNER_PID
        let table = &*(buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_PID);
        let rows = table.table.as_ptr(); // Pointer to the array of MIB_TCPROW_OWNER_PID
        let count = table.dwNumEntries; // Number of entries in the table

        for i in 0..count {
            let row = &*rows.add(i as usize); // Access each row

            // Convert the port from big-endian
            let port = u16::from_be(row.dwLocalPort as u16);

            // Generate the unique ID
            let id = generate_unique_id(row.dwOwningPid, port);

            // Extract data for PortInfo
            let port_info = PortInfo {
                id, // Set the generated unique ID
                port, // Convert port from big-endian
                pid: row.dwOwningPid,
                process_name: String::new(), // Placeholder (to be retrieved later)
                process_path: String::new(), // Placeholder (to be retrieved later)
                is_listener: row.dwState == TCP_STATE_LISTEN, // Check if state is LISTEN
            };


            if !results.iter().any(|entry: &PortInfo| entry.port == port && entry.pid == row.dwOwningPid) {
                results.push(port_info);
            }

        }
    }

    results
}


/// Parse the TCP/IPv6 table buffer into a list of PortInfo
fn parse_tcp_ipv6(buffer: &[u8]) -> Vec<PortInfo> {
    let mut results = Vec::new();

    unsafe {
        // Interpret the buffer as a pointer to MIB_TCP6TABLE_OWNER_PID
        let table = &*(buffer.as_ptr() as *const MIB_TCP6TABLE_OWNER_PID);
        let rows = table.table.as_ptr(); // Pointer to the array of MIB_TCP6ROW_OWNER_PID
        let count = table.dwNumEntries; // Number of entries in the table

        for i in 0..count {
            let row = &*rows.add(i as usize); // Access each row


            // Convert the port from big-endian
            let port = u16::from_be(row.dwLocalPort as u16);

            // Generate the unique ID
            let id = generate_unique_id(row.dwOwningPid, port);

            // Extract data for PortInfo
            let port_info = PortInfo {
                id,
                port,
                pid: row.dwOwningPid,
                process_name: String::new(), // Placeholder (to be retrieved later)
                process_path: String::new(), // Placeholder (to be retrieved later)
                is_listener: row.dwState == TCP_STATE_LISTEN, // Check if state is LISTEN
            };


            if !results.iter().any(|entry: &PortInfo| entry.port == port && entry.pid == row.dwOwningPid) {
                results.push(port_info);
            }

        }
    }

    results
}



/// Parse the UDP/IPv4 table buffer into a list of PortInfo
fn parse_udp_ipv4(buffer: &[u8]) -> Vec<PortInfo> {
    let mut results = Vec::new();

    unsafe {
        // Interpret the buffer as a pointer to MIB_UDPTABLE_OWNER_PID
        let table = &*(buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_PID);
        let rows = table.table.as_ptr(); // Pointer to the array of MIB_UDPROW_OWNER_PID
        let count = table.dwNumEntries; // Number of entries in the table

        for i in 0..count {
            let row = &*rows.add(i as usize); // Access each row


            // Convert the port from big-endian
            let port = u16::from_be(row.dwLocalPort as u16);

            // Generate the unique ID
            let id = generate_unique_id(row.dwOwningPid, port);

            // Extract data for PortInfo
            let port_info = PortInfo {
                id,
                port,
                pid: row.dwOwningPid,
                process_name: String::new(), // Placeholder (to be retrieved later)
                process_path: String::new(), // Placeholder (to be retrieved later)
                is_listener: false,         // UDP does not have a LISTEN state
            };


            if !results.iter().any(|entry: &PortInfo| entry.port == port && entry.pid == row.dwOwningPid) {
                results.push(port_info);
            }

        }
    }

    results
}



/// Parse the UDP/IPv6 table buffer into a list of PortInfo
fn parse_udp_ipv6(buffer: &[u8]) -> Vec<PortInfo> {
    let mut results = Vec::new();

    unsafe {
        // Interpret the buffer as a pointer to MIB_UDP6TABLE_OWNER_PID
        let table = &*(buffer.as_ptr() as *const MIB_UDP6TABLE_OWNER_PID);
        let rows = table.table.as_ptr(); // Pointer to the array of MIB_UDP6ROW_OWNER_PID
        let count = table.dwNumEntries; // Number of entries in the table

        for i in 0..count {
            let row = &*rows.add(i as usize); // Access each row

            // Convert the port from big-endian
            let port = u16::from_be(row.dwLocalPort as u16);

            // Generate the unique ID
            let id = generate_unique_id(row.dwOwningPid, port);

            // Extract data for PortInfo
            let port_info = PortInfo {
                id,
                port,
                pid: row.dwOwningPid,
                process_name: String::new(), // Placeholder (to be retrieved later)
                process_path: String::new(), // Placeholder (to be retrieved later)
                is_listener: false,         // UDP does not have a LISTEN state
            };



            if !results.iter().any(|entry: &PortInfo| entry.port == port && entry.pid == row.dwOwningPid) {
                results.push(port_info);
            }
        }
    }

    results
}


/// Retrieve and parse all TCP and UDP connections
pub fn fetch_ports() -> Result<Vec<crate::common::PortInfo>, String> {
    let protocols = [
        Protocol::TcpIpv4,
        Protocol::TcpIpv6,
        Protocol::UdpIpv4,
        Protocol::UdpIpv6,
    ];

    let mut all_connections = Vec::new();

    for protocol in protocols {
        match get_buffer_size(&protocol) {
            Some(buffer_size) => {
                if let Some(buffer) = fetch_table(&protocol, buffer_size) {
                    match protocol {
                        Protocol::TcpIpv4 => {
                            all_connections.extend(parse_tcp_ipv4(&buffer));
                        }
                        Protocol::TcpIpv6 => {
                            all_connections.extend(parse_tcp_ipv6(&buffer));
                        }
                        Protocol::UdpIpv4 => {
                            all_connections.extend(parse_udp_ipv4(&buffer));
                        }
                        Protocol::UdpIpv6 => {
                            all_connections.extend(parse_udp_ipv6(&buffer));
                        }
                    }
                } else {
                    return Err(format!(
                        "Failed to fetch table for protocol: {:?}",
                        protocol
                    ));
                }
            }
            None => {
                return Err(format!(
                    "Failed to get buffer size for protocol: {:?}",
                    protocol
                ));
            }
        }
    }

    Ok(all_connections)
}




pub fn kill_process(pid: u32) -> KillProcessResponse {
    unsafe {
        // Try to open the process with terminate access
        match OpenProcess(PROCESS_TERMINATE, false, pid) {
            Ok(process_handle) => {
                // Try to terminate the process
                let terminate_result = TerminateProcess(process_handle, 1);
                let _ = CloseHandle(process_handle); // Ensure the handle is closed regardless of the result

                match terminate_result {
                    Ok(()) => KillProcessResponse {
                        success: true,
                        message: format!("Successfully killed process with PID {}", pid),
                    },
                    Err(error) => {
                        let message = if error.code() == ERROR_ACCESS_DENIED.into() {
                            "Access denied".to_string()
                        } else {
                            format!("Error code: {:?}", error.code())
                        };
                        KillProcessResponse {
                            success: false,
                            message: format!("Failed to terminate process with PID {}: {}", pid, message),
                        }
                    }
                }
            }
            Err(error) => KillProcessResponse {
                success: false,
                message: format!(
                    "Failed to open process with PID {}: {}",
                    pid, error.message()
                ),
            },
        }
    }
}
