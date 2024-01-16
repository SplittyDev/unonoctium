use std::sync::Arc;

use parking_lot::RwLock;

use super::Player;

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub players: Vec<Arc<RwLock<Player>>>,
}
