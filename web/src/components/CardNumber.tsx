type Props = {
    number: number
}

export default function CardNumber({ number }: Props) {
    return (
        <div className="text-7xl font-bold text-white p-4">{number}</div>
    )
}
