#[derive(Debug, serde::Serialize)]
pub struct Host {
    pub ip: String,
    pub ports: Vec<Port>,
}

#[derive(Debug, serde::Serialize)]
pub struct Port {
    pub port: u16,
    pub service: Option<String>,
    pub version: Option<String>,
}
