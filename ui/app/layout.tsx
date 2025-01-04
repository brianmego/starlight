import type { Metadata } from "next";
import { Providers } from "./providers";
import { Inter } from "next/font/google";
import "@/app/ui/globals.css";

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
    const data = {
        API_ROOT: process.env.API_ROOT
    }
    return (
        <html lang="en">
            <body className={`${inter.className} antialiased`}>
                <Providers>{children}</Providers>
                <script dangerouslySetInnerHTML={{
                __html: `window.ENV = ${JSON.stringify(data)}`,
                }}
                />
            </body>
        </html>
    );
}
