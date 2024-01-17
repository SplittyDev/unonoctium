type Props = {
    text: string
}

export default function CardText({ text }: Props) {
    return (
        <div className="text-4xl font-bold text-white p-4 text-center">{text}</div>
    )
}
