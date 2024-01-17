use serde::{Deserialize, Serialize};

#[macro_use]
mod macros;

mod base;
mod client;
mod server;
mod traits;

pub use base::BaseEvent;
pub use server::*;
pub use traits::*;

event! {
    client:
        Pong,
        CreateLobby,
        JoinLobby

    server:
        Ping,
        LobbyCreated,
        LobbyJoined
}
