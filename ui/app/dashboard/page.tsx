'use client';
import useSWR, { useSWRConfig } from 'swr';
import React, { Key, useEffect, useState, useContext } from "react";
import { AllSelections, LockedData, ResLocation, ResDate, ResDay, ResTime } from '../lib/definitions';
import { Button, Listbox, ListboxItem, Spacer } from "@nextui-org/react";
import { ListboxWrapper } from "./ListboxWrapper";
import { getCookie } from 'cookies-next'

export default function Page() {
    const jwt = getCookie('jwt')?.toString()

    const [filteredDate, setFilteredDate] = useState(undefined);
    const [filteredLocation, setFilteredLocation] = useState(undefined);
    const [filteredDay, setFilteredDay] = useState(undefined);
    const [filteredTime, setFilteredTime] = useState(undefined);
    const [isReservable, setIsReservable] = useState(false);

    const [dates, setDates] = useState<Array<ResDate>>([]);
    const [thisWeekDates, setThisWeekDates] = useState<Array<ResDate>>([]);
    const [nextWeekDates, setNextWeekDates] = useState<Array<ResDate>>([]);
    const [locations, setLocations] = useState<Array<ResLocation>>([]);
    const [startTimes, setStartTimes] = useState<Array<ResTime>>([]);
    const fetcher = (...args) => fetch(...args).then(res => res.json());
    const { data, error, isLoading } = useSWR(`http://0:1912/api/reservation`, fetcher);

    useEffect(() => {
        if (data) {
            let filteredData = filteredDay ? data.filter(x => x.day_of_week_id == filteredDay) : data;
            filteredData = filteredDate ? filteredData.filter(x => x.date == filteredDate) : filteredData;
            filteredData = filteredLocation ? filteredData.filter(x => x.location_id == filteredLocation) : filteredData;
            filteredData = filteredTime ? filteredData.filter(x => x.start_time_id == filteredTime) : filteredData;
            let this_week_dates = [...new Map(filteredData.filter(x => x.next_week === false).map(x => [x.date, { key: x.date, value: `${x.date} (${x.day_of_week_name})` }])).values()];
            let next_week_dates = [...new Map(filteredData.filter(x => x.next_week === true).map(x => [x.date, { key: x.date, value: `${x.date} (${x.day_of_week_name})` }])).values()];
            let dates = [...new Map(filteredData.map(x => [x.date, { key: x.date, value: `${x.date} (${x.day_of_week_name})` }])).values()];
            let locations = [...new Map(filteredData.map(x => [x.location_id, { key: x.location_id, value: x.location_name }])).values()];
            let times = [...new Map(filteredData.map(x => [x.start_time_id, { key: x.start_time_id, value: x.start_time_name }])).values()]
            locations.sort((a, b) => { { return (a.value > b.value) - (a.value < b.value) } });
            dates.sort((a, b) => { { return (a.value > b.value) - (a.value < b.value) } });
            times.sort((a, b) => { { return (a.key > b.key) - (a.key < b.key) } });
            setDates(dates);
            setThisWeekDates(this_week_dates);
            setNextWeekDates(next_week_dates);
            setLocations(locations);
            setStartTimes(times)
        }
    }, [data, filteredDate, filteredLocation, filteredDay, filteredTime]);

    useEffect(() => {
        if (dates.length == 1 && locations.length == 1 && startTimes.length == 1) {
            setIsReservable(true);
        } else {
            setIsReservable(false);
        }
    }, [dates, locations, startTimes])

    async function handleReserve() {
        const allSelections: AllSelections = {
            "location": locations[0].key,
            "date": dates[0].key,
            "startTime": startTimes[0].key,
            "jwt": jwt
        };
        const reservation_id = data.filter(x => x.location_id == allSelections.location && x.start_time_id == allSelections.startTime && x.date == allSelections.date)[0].reservation_id;

        await fetch(`http://0:1912/api/reservation/${reservation_id}`, {
            method: "POST",
            headers: {
                "authorization": `Bearer ${jwt}`
            },
        }).then(res => {
            if (res.status == 401) {
                alert("This session is no longer valid. Please log in again.")
            } else if (res.status == 200) {
                alert("Reserved!")

                setFilteredDate(undefined);
                setFilteredLocation(undefined);
                setFilteredDay(undefined);
                setFilteredTime(undefined);
            }
        })
    }


    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>

    return (
        <>
            <h1><b>Dashboard Page</b></h1>
            <div className="flex gap-2">
                <div >
                    <EndpointListbox label="This Week" setter={setFilteredDate} data={thisWeekDates} />
                    <EndpointListbox label="Next Week" setter={setFilteredDate} data={nextWeekDates} />
                </div>
                <EndpointListbox label="Locations" setter={setFilteredLocation} data={locations} />
                <EndpointListbox label="Time" setter={setFilteredTime} data={startTimes} />
            </div>
            <Spacer y={4} />
            <ReserveButton clickHandler={handleReserve} isDisabled={!isReservable} />
        </>
    )
}

function ReserveButton({ clickHandler, isDisabled }) {
    return (
        <Button color="primary" isDisabled={isDisabled} onPress={clickHandler}>Reserve</Button>
    )
}

function EndpointListbox({ label, setter, data }) {
    const [selectedKeys, setSelectedKeys] = useState(new Set([]));
    const selectedValue = React.useMemo(
        () => Array.from(selectedKeys).join(", "),
        [selectedKeys]
    )

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
        <div className="flex-none border px-2 py-4">
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
