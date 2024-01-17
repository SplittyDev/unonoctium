/// A macro helper for defining events.
/// 
/// This macro is used to define events in a way that is both concise and easy to use.  
/// It enables the creation of both client and server events.
/// 
/// Client events are events that are sent from the client to the server.
/// - They only have to implement `serde::Deserialize`.
/// - The macro implements `IntoBaseEvent` for easy conversion into a `BaseEvent`.
/// 
/// Server events are events that are sent from the server to the client.
/// - They only have to implement `serde::Serialize`.
/// - The macro implements `FromBaseEvent` for easy conversion from a `BaseEvent`.
/// 
/// ## Example
/// This example assumes that the event data structs are already defined.
/// 
/// ```rs
/// event! {
///     client:
///         MovePlayerRequest,
///         DeleteItemRequest
/// 
///     server:
///         PlayerPositionUpdate,
///         InventoryUpdate
/// }
/// ```
macro_rules! event {

    // Multi-line event definition
    (client: $($clientevent:ident),* server: $($serverevent:ident),* $(,)?) => {
        $(event! { client $clientevent })*
        $(event! { server $serverevent })*
    };

    // Client event definition
    (client $name:ident) => {
        paste::paste! {
            #[derive(Debug, Deserialize)]
            pub struct [<$name Event>] {
                pub id: &'static str,
                pub data: crate::events::client::$name,
            }

            impl crate::events::FromBaseEvent for [<$name Event>] {
                fn from_base_event(event: crate::events::base::BaseEvent) -> anyhow::Result<Self> {
                    if event.id != Self::ID {
                        anyhow::bail!("Invalid event ID: {}", event.id);
                    }

                    Ok(Self {
                        id: Self::ID,
                        data: serde_json::from_value(event.data.into_owned())?,
                    })
                }
            }
        }

        event!{ __impl $name }
    };

    // Server event definition
    (server $name:ident) => {
        paste::paste! {
            #[derive(Debug, Serialize)]
            pub struct [<$name Event>] {
                pub data: crate::events::server::$name,
            }

            impl crate::events::server::$name {

                #[allow(dead_code)]
                pub fn into_event(self) -> crate::events::base::BaseEvent {
                    let event = [<$name Event>] { data: self };
                    event.into_base_event()
                }
            }

            impl crate::events::IntoBaseEvent for [<$name Event>] {
                fn into_base_event(self) -> crate::events::base::BaseEvent {
                    crate::events::base::BaseEvent::from(self)
                }
            }
        }
        event!{ __impl $name }
    };

    // Private implementation details
    (__impl $name:ident) => {
        paste::paste! {
            impl crate::events::Event for [<$name Event>] {
                const ID: &'static str = stringify!([<$name Event>]);
            }
        }
    };
}
