import type { Metadata, Viewport } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'

const inter = Inter({ subsets: ['latin'] })

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  minimumScale: 1,
  viewportFit: 'cover',
}

export const metadata: Metadata = {
  title: 'Unonoctium',
  description: 'A multiplayer card game',
  manifest: '/manifest.webmanifest',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <main className="h-full w-full pb-[2rem] flex flex-col items-center gap-[2rem] md:gap-[4rem] md:px-4 md:max-w-2xl lg:max-w-3xl xl:max-w-4xl">
          {children}
        </main>
      </body>
    </html>
  )
}
