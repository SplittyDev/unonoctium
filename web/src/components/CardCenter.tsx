type Props = {
    children: React.ReactNode
}

export default function CardCenter({ children }: Props) {
    return (
        <div className="w-full h-full flex items-center justify-center rotate-[-45deg]">
            {children}
        </div>
    )
}
