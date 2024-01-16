use std::sync::Arc;

use parking_lot::RwLock;
use rand::Rng;

use super::{game::Game, IdGenerator, Player};

#[derive(Debug)]
pub struct Lobby {
    pub id: String,
    pub players: Vec<Arc<RwLock<Player>>>,
    pub games: Arc<RwLock<Vec<Game>>>,
}

impl Lobby {
    pub fn new(id: String, creator: Player) -> Self {
        Self {
            id,
            players: vec![Arc::new(RwLock::new(creator))],
            games: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_player(&mut self, player: Arc<RwLock<Player>>) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: &Arc<RwLock<Player>>) {
        self.players.retain(|p| !Arc::ptr_eq(p, player));
    }
}

impl IdGenerator for Lobby {
    fn generate_id() -> String {
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }
}
