use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use futures_util::StreamExt;
use log::*;
use parking_lot::{RwLock, Mutex};
use tokio::net::{TcpListener, TcpStream};

mod events;
mod state;

use state::{Game, Lobby, Player};

use crate::{
    events::{CreateLobbyEvent, Event, FromBaseEvent as _, LobbyCreated, PongEvent},
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
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Accepting websocket connection failed");

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        info!("[Server] Accepted client: {}", peer);

        let mut ping_interval = tokio::time::interval(Duration::from_millis(5000));

        let ping_timeout = Duration::from_secs(60);
        let last_ping_response = Arc::new(Mutex::new(tokio::time::Instant::now()));

        loop {
            tokio::select! {
                msg = ws_receiver.next() => {
                    let Some(msg) = msg else { break };
                    let msg = msg?;

                    // Only handle text messages for now
                    let Ok(msg) = msg.to_text() else { continue };

                    // Parse base event
                    let event = serde_json::from_str::<events::BaseEvent>(msg)
                        .context("Unable to deserialize event")?;

                    info!("[Client -> Server]: {}", event.id);

                    // Handle specific event
                    match event.id.as_ref() {
                        // Pong
                        PongEvent::ID => {
                            let _ = PongEvent::from_base_event(event)?;

                            // Update last ping response
                            *last_ping_response.lock() = tokio::time::Instant::now();
                        }

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
                                .send(&mut ws_sender)
                                .await?;
                        }

                        // Unknown event
                        _ => {
                            warn!("Unknown event ID: {}", event.id);
                        }
                    }
                },

                _ = ping_interval.tick() => {
                    // Check if we've received a ping response within the timeout
                    let last_ping_response = *last_ping_response.lock();
                    if tokio::time::Instant::now() - last_ping_response > ping_timeout {
                        warn!("Client {} timed out", peer);
                        break;
                    }

                    events::Ping {}.into_event().send(&mut ws_sender).await?;
                }
            }
        }

        println!("Client {} disconnected", peer);

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
