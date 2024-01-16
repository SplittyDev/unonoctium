use std::net::SocketAddr;

use rand::Rng;

use super::IdGenerator;

#[derive(Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub addr: SocketAddr,
}

impl Player {
    pub fn new(id: String, name: Option<String>, addr: SocketAddr) -> Self {
        Self {
            id,
            name: name.unwrap_or_else(|| "Anonymous".to_string()),
            addr,
        }
    }
}

impl IdGenerator for Player {
    fn generate_id() -> String {
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}
