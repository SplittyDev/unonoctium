// import { Socket, io } from "socket.io-client"

import type { BaseEvent, AnyBaseEvent } from "./BaseEvent"
import { CreateLobbyEvent, LobbyCreatedEvent, PingEvent, PongEvent, ServerEventIds } from './Event'

export default class Client {
    private socket: WebSocket

    constructor() {
        this.socket = new WebSocket("ws://localhost:9002")

        this.socket.onopen = () => {
            console.log("Connected to server");
        }

        this.socket.onclose = () => {
            console.log("Disconnected from server");
        }

        this.socket.onerror = (error) => {
            console.error("WebSocket error:", error);
        }

        this.socket.onmessage = (event) => {
            console.log("Message from server:", event.data);
            const baseEvent = JSON.parse(event.data) as AnyBaseEvent;
            const parse = <T extends AnyBaseEvent>(event: AnyBaseEvent) => event.data as T["data"];
            switch (baseEvent.id as ServerEventIds) {
                case "PingEvent": {
                    let data = parse<PingEvent>(baseEvent)
                    this.pong()
                    break;
                }
                case "LobbyCreatedEvent": {
                    let data = parse<LobbyCreatedEvent>(baseEvent)
                    break;
                }
                default: {
                    console.error("Unknown event", baseEvent)
                }
            }
        }
    }

    private send<T extends AnyBaseEvent>(data: T) {
        this.socket.send(JSON.stringify(data))
    }

    private pong() {
        this.send<PongEvent>({
            id: "PongEvent",
            data: {}
        })
    }

    public createLobby() {
        this.send<CreateLobbyEvent>({
            id: "CreateLobbyEvent",
            data: {}
        })
    }
}
