export type BaseEvent<ID extends string, T> = {
    id: ID,
    data: T
}

export type AnyBaseEvent = BaseEvent<string, any>
