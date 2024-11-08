import type { Metadata } from "next";
import { Providers } from "./providers";
import { Inter } from "next/font/google";
import "./ui/globals.css";
import SocketProvider from './socket-provider'

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
    title: "Cookie Monsters",
    description: "Starlight Service Unit cookie signup",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body className={`${inter.className} antialiased`}>
                <SocketProvider>{children}</SocketProvider>
            </body>
        </html>
    );
}
