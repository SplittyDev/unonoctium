use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LobbyCreated {
    pub lobby_id: String,
}

#[derive(Debug, Serialize)]
pub struct LobbyJoined {
    pub lobby_id: String,
    pub player_name: String,
}
