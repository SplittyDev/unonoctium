use super::BaseEvent;

/// An event with a static ID.
pub trait Event {
    /// The static ID of the event.
    const ID: &'static str;

    /// Returns the static ID of the event.
    fn id(&self) -> &'static str {
        Self::ID
    }
}

/// A trait for converting a `BaseEvent` into a specific event.
pub trait FromBaseEvent {
    /// Attempts to create the event from a `BaseEvent`.
    fn from_base_event(event: BaseEvent) -> anyhow::Result<Self>
    where
        Self: Sized;
}

/// A trait for converting a specific event into a `BaseEvent`.
pub trait IntoBaseEvent {
    /// Converts the event into a `BaseEvent`.
    fn into_base_event(self) -> BaseEvent;
}
