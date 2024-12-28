#[derive(serde::Serialize, Debug)]
pub struct PortInfo {
    pub id: String,
    pub port: String,
    pub pid: u32,
    pub process_name: String,
    pub process_path: String,
}

#[derive(serde::Serialize, Debug)]
pub struct KillProcessResponse {
    pub success: bool,
    pub message: String,
}
