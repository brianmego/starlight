'use client';
import useSWR from 'swr';
import React, { useEffect, useState } from "react";
import { AllSelections, LockedData } from '../lib/definitions';
import { Button, Listbox, ListboxItem } from "@nextui-org/react";
import { ListboxWrapper } from "./ListboxWrapper";
import { io, Socket } from 'socket.io-client';
import { getCookie } from 'cookies-next'

export default function Page() {
    const socket = io("ws://192.168.1.190:1912/ws");
    const [lockedData, setLockedData]: [LockedData, any] = useState({ locations: [] });
    const jwt = getCookie('jwt')?.toString()
    const [selectedLocation, setSelectedLocation] = useState(undefined);
    const [selectedDay, setSelectedDay] = useState(undefined);
    const [selectedTimeslot, setSelectedTimeslot] = useState(undefined);
    console.log("Hey")

    useEffect(() => {
        socket.on('locked-data', (msg) => {
            setLockedData(msg);
        })
        return () => {
            socket.off("locked-data");
        };
    }, []);

    function handleReserve() {
        socket.emit("reserve", "hey")
    }
    function handleClick(endpoint: Endpoint, selectedValue: string) {
        endpoint.setter(selectedValue);
        const allSelections: AllSelections = {"location": selectedLocation, "day": selectedDay, "timeslot": selectedTimeslot};
        socket.send({ "endpoint": endpoint.endpoint, "value": selectedValue, "jwt": jwt, "allSelections": allSelections });
    }

    return (
        <>
            <h1><b>Dashboard Page</b></h1>
            <div className="flex gap-2">
                <EndpointListbox endpoint={new Endpoint("location", "Locations", setSelectedLocation)} clickHandler={handleClick} />
            </div>
            <div className="flex gap-2">
                <EndpointListbox endpoint={new Endpoint("dayofweek", "Days", setSelectedDay)} clickHandler={handleClick} />
            </div>
            <div className="flex gap-2">
                <EndpointListbox endpoint={new Endpoint("timeslot", "Timeslots", setSelectedTimeslot, x => x.start, x => { return `${x.start} - ${x.end}` })} clickHandler={handleClick} />
            </div>
            <ReserveButton clickHandler={handleReserve}/>
        </>
    )
}

function ReserveButton(clickHandler: Function) {
    return (
        <Button color="primary" onClick={clickHandler}>Reserve</Button>
    )
}
class Endpoint {
    constructor(public endpoint: string, public aria_label: string, public setter: Function, public keyFunc = null, public valFunc = null) {
        this.endpoint = endpoint;
        this.aria_label = aria_label;
        this.setter = setter;
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
function EndpointListbox({ endpoint, clickHandler }) {
    const fetcher = (...args) => fetch(...args).then(res => res.json())
    const { data, error, isLoading } = useSWR(`http://192.168.1.190:1912/api/${endpoint.endpoint}`, fetcher)
    const [selectedKeys, setSelectedKeys] = useState(new Set(["text"]));
    const selectedValue = React.useMemo(
        () => Array.from(selectedKeys).join(", "),
        [selectedKeys]
    )
    const [lockedData, setLockedData]: [[String], any] = useState([]);

    // useEffect(() => {
    //     socket.on('locked-data', (msg: LockedData) => {
    //         setLockedData(msg.locations.map(x => x.name));
    //     })
    //     return () => {
    //         socket.off("message");
    //     };
    // }, [socket]);

    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>
    return (
        <div className="flex-none">
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
                            return <ListboxItem
                                key={endpoint.keyFunc(x)}
                                onClick={() => { clickHandler(endpoint, selectedValue) }}
                                isDisabled={lockedData.includes(endpoint.keyFunc(x))}>{endpoint.valFunc(x)}
                            </ListboxItem>
                        }
                        )
                    }
                </Listbox>
            </ListboxWrapper>
        </div>

    )
}
