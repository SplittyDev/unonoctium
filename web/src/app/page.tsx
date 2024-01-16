import UnoCard from '@/components/UnoCard'
import Image from 'next/image'

export default function Home() {
  return (
    <main className="flex flex-col items-center justify-between p-24">
      <div className="flex">
        <UnoCard hue={0} />
        <UnoCard hue={45} />
        <UnoCard hue={90} />
        <UnoCard hue={135} />
        <UnoCard hue={180} />
        <UnoCard hue={225} />
        <UnoCard hue={270} />
        <UnoCard hue={315} />
      </div>
    </main>
  )
}
