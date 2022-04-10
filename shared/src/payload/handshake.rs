use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HandshakeStatus {
    pub address: String,
    pub port: u16,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HandshakeResponse {
    pub status: String,
}

impl HandshakeStatus {
    pub fn new(address: &str, port: u16, username: &str) -> Self {
        Self {
            address: address.to_owned(),
            port,
            username: username.to_owned(),
        }
    }
}

impl HandshakeResponse {
    pub fn new(status: &str) -> Self {
        Self {
            status: status.to_owned(),
        }
    }
}