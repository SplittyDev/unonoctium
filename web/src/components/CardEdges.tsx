type Props = {
    children: React.ReactNode
}

export default function CardEdges({ children }: Props) {
    return (
        <>
            <div className="absolute top-0 left-0">
                {children}
            </div>
            <div className="absolute bottom-0 right-0">
                {children}
            </div>
        </>
    )
}
