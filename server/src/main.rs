use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use futures_util::StreamExt;
use log::*;
use parking_lot::RwLock;
use tokio::net::{TcpListener, TcpStream};

mod events;
mod state;

use state::{Game, Lobby, Player};

use crate::{
    events::{CreateLobbyEvent, Event, FromBaseEvent as _, LobbyCreated},
    state::IdGenerator,
};

#[derive(Debug, Default, Clone)]
struct GameServer {
    lobbies: Arc<RwLock<HashMap<String, Lobby>>>,
}

unsafe impl Send for GameServer {}

impl GameServer {
    pub fn new() -> Self {
        GameServer::default()
    }

    pub async fn run(self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(&addr)
            .await
            .expect("Unable to bind to listen address");
        info!("Listening on {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("Connected streams should have a peer address");
            info!("Peer address: {}", peer);

            let this = self.clone();
            tokio::spawn(async move {
                _ = this.accept_connection(peer, stream).await;
            });
        }

        Ok(())
    }

    async fn accept_connection(&self, peer: SocketAddr, stream: TcpStream) -> Result<()> {
        let mut ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Accepting websocket connection failed");

        info!("[Server] Accepted client: {}", peer);

        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;

            // Only handle text messages for now
            let Ok(msg) = msg.to_text() else { continue };

            // Parse base event
            let event = serde_json::from_str::<events::BaseEvent>(msg)
                .context("Unable to deserialize event")?;

            info!("[Client -> Server]: {}", event.id);

            // Handle specific event
            match event.id.as_ref() {
                // Create lobby
                CreateLobbyEvent::ID => {
                    _ = CreateLobbyEvent::from_base_event(event)?;

                    // Create player
                    let player_id = Player::generate_id();
                    let player = Player::new(player_id, None, peer);

                    // Create lobby
                    let lobby_id = Lobby::generate_id();
                    let lobby = Lobby::new(lobby_id.clone(), player);

                    // Write lobby to server state
                    self.lobbies.write().insert(lobby.id.clone(), lobby);

                    // Send response
                    LobbyCreated { lobby_id }
                        .into_event()
                        .send(&mut ws_stream)
                        .await?;
                }

                // Unknown event
                _ => {
                    warn!("Unknown event ID: {}", event.id);
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let addr: SocketAddr = "127.0.0.1:9002".parse().context("Invalid address")?;
    let server = GameServer::new();
    server.run(addr).await?;

    Ok(())
}
