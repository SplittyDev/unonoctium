use std::borrow::Cow;

use anyhow::Context;
use futures_util::SinkExt;
use log::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use super::Event;

/// A base event with formless data.
///
/// This is the type that is sent over the wire.
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseEvent {
    pub id: Cow<'static, str>,
    pub data: Cow<'static, serde_json::Value>,
}

impl BaseEvent {
    /// Sends the event to a client.
    pub async fn send(self, stream: &mut WebSocketStream<TcpStream>) -> anyhow::Result<()> {
        info!("[Server -> Client]: {}", &self.id);

        // Send the event to the client
        stream
            .send(self.try_into()?)
            .await
            .context("Unable to send message to client")
    }
}

// Enables conversion from any specific event type into a base event.
impl<T> From<T> for BaseEvent
where
    T: Event + Serialize,
{
    fn from(event: T) -> Self {
        Self {
            id: event.id().into(),
            data: Cow::Owned(serde_json::to_value(event).expect("Unable to serialize event")),
        }
    }
}

// Enables conversion from a base event into a tungstenite message.
impl TryInto<Message> for BaseEvent {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Message, Self::Error> {
        Ok(Message::Text(
            serde_json::to_string(&self).context("Unable to serialize event")?,
        ))
    }
}
