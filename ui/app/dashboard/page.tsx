'use client';
import useSWR from 'swr';
import React, { useEffect, useState } from "react";
import { Listbox, ListboxItem } from "@nextui-org/react";
import { ListboxWrapper } from "./ListboxWrapper";
import { getLocations } from "@/app/api/locations";
import { io, Socket } from 'socket.io-client';


export default function Page() {
    const socket = io("ws://192.168.1.190:1912");
    const [data, setData] = useState(null);

    useEffect(() => {
        socket.on('message-back', (msg) => {
            setData(msg);
        })
        return () => {
            socket.off("message");
        };
    }, [socket]);

    function handleClick(socket: Socket, endpoint, selectedValue) {
        socket.send({ "endpoint": endpoint, "value": selectedValue });
    }

    return (
        <>
            <h1><b>Dashboard Page</b></h1>
            <p>{data}</p>
            <div className="flex gap-2">
                <EndpointListbox endpoint={new Endpoint("location", "Locations")} clickHandler={handleClick} socket={socket} />
            </div>
        </>
    )
    // <EndpointListbox endpoint={new Endpoint("dayofweek", "Days")} onClick={() => handleClick(socket)} />
    // <EndpointListbox endpoint={new Endpoint(
    //     "timeslot",
    //     "Timeslots",
    //     x => x.start,
    //     x => { return `${x.start} - ${x.end}` }
    // )}
    //     onClick={() => handleClick(socket)} />
}

class Endpoint {
    constructor(public endpoint: string, public aria_label: string, public keyFunc = null, public valFunc = null) {
        this.endpoint = endpoint;
        this.aria_label = aria_label;
        if (keyFunc === null) {
            this.keyFunc = this.defaultKeyFunc;
        } else {
            this.keyFunc = keyFunc;
        }
        if (valFunc === null) {
            this.valFunc = this.defaultValFunc;
        } else {
            this.valFunc = valFunc;
        }

    }

    defaultKeyFunc(row) {
        return row.name;
    }
    defaultValFunc(row) {
        return row.name;
    }
}
function EndpointListbox({ endpoint, clickHandler, socket }) {
    const fetcher = (...args) => fetch(...args).then(res => res.json())
    const { data, error, isLoading } = useSWR(`http://192.168.1.190:1912/api/${endpoint.endpoint}`, fetcher)
    const [selectedKeys, setSelectedKeys] = useState(new Set(["text"]));
    const selectedValue = React.useMemo(
        () => Array.from(selectedKeys).join(", "),
        [selectedKeys]
    )
    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>
    return (
        <div class="flex-none">
            <p>{endpoint.aria_label}</p>
            <ListboxWrapper>
                <Listbox
                    aria-label={endpoint.aria_label}
                    variant="flat"
                    selectionMode="single"
                    selectedKeys={selectedKeys}
                    onSelectionChange={setSelectedKeys}
                >
                    {
                        data.map(x => {
                            return <ListboxItem key={endpoint.keyFunc(x)} onClick={() => { clickHandler(socket, endpoint.endpoint, selectedValue) }}>{endpoint.valFunc(x)}</ListboxItem>
                        }
                        )
                    }
                </Listbox>
            </ListboxWrapper>
        </div>

    )
}