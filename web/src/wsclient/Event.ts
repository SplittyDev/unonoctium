import { BaseEvent } from "./BaseEvent"

// Client events

export type PongEvent = BaseEvent<"PongEvent", {}>
export type CreateLobbyEvent = BaseEvent<"CreateLobbyEvent", {}>

// Server events

export type PingEvent = BaseEvent<"PingEvent", {}>
export type LobbyCreatedEvent = BaseEvent<"LobbyCreatedEvent", {
    lobby_id: string
}>

export type ServerEventIds = PingEvent["id"] | LobbyCreatedEvent["id"]
