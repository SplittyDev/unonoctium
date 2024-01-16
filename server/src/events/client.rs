use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, TS, Deserialize)]
#[ts(export)]
pub struct CreateLobby;

#[derive(Debug, TS, Deserialize)]
#[ts(export)]
pub struct JoinLobby {
    lobby_name: String,
    player_name: Option<String>,
}
