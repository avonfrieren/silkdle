use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Zone {
    pub name: String,
    pub size: u8,
    pub acte: u8,
    pub bosses: u8,
    pub station: bool,
}

#[derive(Debug, Deserialize)]
pub struct ZonesFile {
    pub zones: Vec<Zone>,
}
