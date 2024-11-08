'use client'

import { createContext } from 'react'
import { io } from 'socket.io-client'
export const SocketContext = createContext(null)

export default function SocketProvider({
    children,
}: {
        children: React.ReactNode
    }) {
    const socket = io("ws://192.168.1.190:1912/ws");
    return <SocketContext.Provider value={socket}>{children}</SocketContext.Provider>
}
