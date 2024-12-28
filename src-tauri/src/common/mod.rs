#[derive(serde::Serialize, Debug)]
pub struct PortInfo {
    pub id: String,
    pub port: String,
    pub pid: u32,
    pub process_name: String,
    pub process_path: String,
}
