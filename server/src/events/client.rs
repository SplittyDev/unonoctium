use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pong { }

#[derive(Debug, Deserialize)]
pub struct CreateLobby {}

#[derive(Debug, Deserialize)]
pub struct JoinLobby {
    lobby_name: String,
    player_name: Option<String>,
}
