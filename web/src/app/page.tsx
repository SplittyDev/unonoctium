'use client'

import CardAction from '@/components/CardAction'
import CardCenter from '@/components/CardCenter'
import CardEdges from '@/components/CardEdges'
import CardNumber from '@/components/CardNumber'
import CardText from "@/components/CardText"
import UnoCard from '@/components/UnoCard'
import Client from '@/wsclient/Client'
import { useEffect, useRef } from 'react'

export default function Home() {
    const client = useRef<Client | null>(null)

    useEffect(() => {
        client.current = new Client()
    }, [])

    const startNewGame = async () => {
        client.current?.createLobby()
    }

    return (
        <div className="flex flex-col items-center p-[2rem] gap-[2rem]">
            <h1 className="text-6xl">Unonoctium</h1>

            <div className="flex flex-row gap-[2rem]">
                <UnoCard hue={200}>
                    <CardEdges>
                        <CardNumber number={0} />
                    </CardEdges>
                    <CardCenter>
                        <CardText text="New Game" />
                    </CardCenter>
                    <CardAction onClick={startNewGame} />
                </UnoCard>
                <UnoCard hue={340}>
                    <CardEdges>
                        <CardNumber number={0} />
                    </CardEdges>
                    <CardCenter>
                        <CardText text="Join Game" />
                    </CardCenter>
                </UnoCard>
            </div>
        </div>
    )
}
