use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Subdomain {
    pub domain: String,
    pub open_ports: Vec<Port>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CertShEntry {
    pub name_value: String,
}
