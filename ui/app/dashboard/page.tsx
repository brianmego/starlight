'use client';
import useSWR from 'swr';
import React, { Key, useEffect, useState, useContext } from "react";
import { AllSelections, LockedData, ResLocation, ResDay, ResTime } from '../lib/definitions';
import { Button, Listbox, ListboxItem } from "@nextui-org/react";
import { ListboxWrapper } from "./ListboxWrapper";
import { io, Socket } from 'socket.io-client';
import { getCookie } from 'cookies-next'
import { SocketContext } from '../socket-provider';
import { scryRenderedDOMComponentsWithClass } from 'react-dom/test-utils';

export default function Page() {
    const socket: Socket = useContext(SocketContext)
    const jwt = getCookie('jwt')?.toString()
    const [selectedLocation, setSelectedLocation] = useState(undefined);
    const [selectedDay, setSelectedDay] = useState(undefined);
    const [selectedStartTime, setStartTime] = useState(undefined);
    const [locations, setLocations] = useState<Array<ResLocation>>([]);
    const [days, setDays] = useState<Array<ResDay>>([]);
    const [startTimes, setStartTimes] = useState<Array<ResTime>>([]);
    const fetcher = (...args) => fetch(...args).then(res => res.json());
    const { data, error, isLoading } = useSWR(`http://192.168.1.190:1912/api/reservation`, fetcher);

    useEffect(() => {
        socket.on('message', (msg: String) => {
            if (msg === "Reserved!") {
                alert!("Reserved!")
            } else if (msg === "This is not reservable") {
                alert!("Not reservable")
            }
        })
        return () => {
            socket.off("message");
        };
    }, [socket]);

    useEffect(() => {
        if (data) {
            let filteredData = selectedDay ? data.filter(x => x.day_of_week_id == selectedDay) : data;
            filteredData = selectedLocation ? filteredData.filter(x => x.location_id == selectedLocation) : filteredData;
            filteredData = selectedStartTime ? filteredData.filter(x => x.start_time_id == selectedStartTime) : filteredData;
            let locations = [...new Map(filteredData.map(x => [x.location_id, { key: x.location_id, value: x.location_name }])).values()];
            let days = [...new Map(filteredData.map(x => [x.day_of_week_id, { key: x.day_of_week_id, value: x.day_of_week_name }])).values()]
            let times = [...new Map(filteredData.map(x => [x.start_time_id, { key: x.start_time_id, value: x.start_time_name }])).values()]
            setLocations(locations);
            setDays(days)
            setStartTimes(times)
        }
    }, [data, selectedLocation, selectedDay, selectedStartTime]);


    function handleReserve() {
        const allSelections: AllSelections = { "location": selectedLocation, "day": selectedDay, "startTime": selectedStartTime, "jwt": jwt };
        socket.emit("reserve", allSelections);
    }

    // function handleClick(endpoint: Endpoint, selectedValue: string) {
    //     endpoint.setter(selectedValue);
    //     socket.send({ "endpoint": endpoint.endpoint, "value": selectedValue, "jwt": jwt });
    // }

    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>


    return (
        <>
            <h1><b>Dashboard Page</b></h1>
            <div className="flex gap-2">
                <EndpointListbox label="Locations" setter={setSelectedLocation} data={locations} />
            </div>
            <div className="flex gap-2">
                <EndpointListbox label="Days" setter={setSelectedDay} data={days} />
            </div>
            <div className="flex gap-2">
                <EndpointListbox label="Time" setter={setStartTime} data={startTimes} />
            </div>
            <ReserveButton clickHandler={handleReserve} />
        </>
    )
}

function ReserveButton({ clickHandler }) {
    return (
        <Button color="primary" onPress={clickHandler}>Reserve</Button>
    )
}

function EndpointListbox({label, setter, data}) {
    // const socket: Socket = useContext(SocketContext)
    const [selectedKeys, setSelectedKeys] = useState(new Set([]));
    const selectedValue = React.useMemo(
        () => Array.from(selectedKeys).join(", "),
        [selectedKeys]
    )
    // const [lockedData, setLockedData]: [[Key], any] = useState([]);

    // useEffect(() => {
    //     socket.on('locked-data', (msg: LockedData) => {
    //         setLockedData(msg[endpoint.data_lock_key].map(x => x.key));
    //     })
    //     return () => {
    //         socket.off("locked-data");
    //     };
    // }, [socket]);

    function selectItem(key) {
        if (key.size === 0) {
            setSelectedKeys(new Set([]));
            setter(null)
        } else {
            setSelectedKeys(key);
            setter(key.currentKey)
        }
    }

    return (
        <div className="flex-none">
            <p>{label}</p>
            <ListboxWrapper>
                <Listbox
                    items={data}
                    aria-label={label}
                    // variant="flat"
                    selectionMode="single"
                selectedKeys={selectedKeys}
                // onSelectionChange={setSelectedKeys}
                onSelectionChange={selectItem}
                // disabledKeys={lockedData}
                // onSelectionChange={(key) => {
                //     setSelectedKeys(key);
                //     // clickHandler(endpoint, key.currentKey)
                // }
                // }
                >
                    {(item) => (
                        <ListboxItem
                            key={item.key}
                        >
                            {item.value}
                        </ListboxItem>
                    )
                    }
                </Listbox>
            </ListboxWrapper>
        </div>

    )
}
