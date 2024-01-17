type Props = {
    onClick: () => void
}

export default function CardAction({ onClick }: Props) {
    return (
        <div className="absolute top-0 left-0 w-full h-full" onClick={onClick} />
    )
}
