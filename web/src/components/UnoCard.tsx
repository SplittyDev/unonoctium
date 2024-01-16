import UnoCardBackground from '@/assets/card.png'

enum CardType {
    Number,
    Skip,
    Reverse,
    DrawTwo,
    Wild,
    WildDrawFour,
}

type Props = {
    hue: number
    type: CardType
}

export default function UnoCard({ hue, type }: Props) {
    const originalWidth = 400
    const originalHeight = 660
    const scale = 0.5

    const width = originalWidth * scale
    const height = originalHeight * scale

    const style = {
        width: `${width}px`,
        height: `${height}px`,
        filter: `hue-rotate(${hue}deg)`,
    }

    return (
        <div className={`relative hover:pb-[1rem] transition-all cursor-pointer`} style={style}>
            <img src={UnoCardBackground.src} alt="Uno card" className="object-contain top-0 left-0 w-full h-full" />
        </div>
    )
}
