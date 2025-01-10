'use client';
import useSWR, { SWRResponse, useSWRConfig } from 'swr';
import React, { useEffect, useState } from "react";
import { AllSelections, ReservationData, ResLocation, ResDate, ResTime } from '../lib/definitions';
import { Button, Listbox, ListboxItem, Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Popover, PopoverTrigger, PopoverContent, Spacer, useDisclosure } from "@nextui-org/react";
import { ListboxWrapper } from "./ListboxWrapper";
import { getCookie } from 'cookies-next'

const fetcher = (url: RequestInfo) => fetch(url).then(res => res.json());

function UserData() {
    let jwt = getCookie('jwt')?.toString();
    if (jwt === undefined) {
        jwt = ""
    }
    const parsed_jwt = JSON.parse(atob(jwt.split('.')[1]));
    const { data, error, isLoading } = useSWR(`${process.env.NEXT_PUBLIC_API_ROOT}/user/${parsed_jwt.ID}`, fetcher);
    const [remainingTokens, setRemainingTokens] = useState(0);
    const [totalTokens, setTotalTokens] = useState(0);

    useEffect(() => {
        if (data) {
            console.log(`Now: ${data.now}`);
            setRemainingTokens(data.total_tokens - data.tokens_used);
            setTotalTokens(data.total_tokens);
        }
    }, [data])

    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>
    return (
        <Popover placement="right">
            <PopoverTrigger>
                <Button>Booth Pick Data</Button>
            </PopoverTrigger>
            <PopoverContent>
                <div className="px-1 py-2">
                    <div className="text-small font-bold">Your troop size grants you a certain number of booth picks for next week&apos;s booths</div>
                    <div className="text-small">Remaining Booth Picks (Next Week): {remainingTokens}</div>
                    <div className="text-tiny">Used Booth Picks: {data.tokens_used}</div>
                    <div className="text-tiny">Total Booth Picks: {totalTokens}</div>
                    <div className="text-tiny">New data unlocks at Noon and 10PM each day (page will auto refresh)</div>
                </div>
            </PopoverContent>
        </Popover>
    );

}
export default function Page() {
    const { mutate } = useSWRConfig()
    const jwt = getCookie('jwt')?.toString();

    const [toggleThisWeekReset, setToggleThisWeekReset] = useState(false);
    const [toggleNextWeekReset, setToggleNextWeekReset] = useState(false);
    const [toggleLocationsReset, setToggleLocationsReset] = useState(false);
    const [toggleTimesReset, setToggleTimesReset] = useState(false);
    const [filteredDate, setFilteredDate] = useState(undefined);
    const [filteredLocation, setFilteredLocation] = useState(undefined);
    const [filteredDay, setFilteredDay] = useState(undefined);
    const [filteredTime, setFilteredTime] = useState(undefined);
    const [isReservable, setIsReservable] = useState(false);
    const { isOpen, onOpen, onOpenChange } = useDisclosure();
    const [modalText, setModalText] = useState("");
    const [modalHeader, setModalHeader] = useState("");

    const [dates, setDates] = useState<Array<ResDate>>([]);
    const [thisWeekDates, setThisWeekDates] = useState<Array<ResDate>>([]);
    const [nextWeekDates, setNextWeekDates] = useState<Array<ResDate>>([]);
    const [locations, setLocations] = useState<Array<ResLocation>>([]);
    const [startTimes, setStartTimes] = useState<Array<ResTime>>([]);
    const { data, error, isLoading }: SWRResponse<ReservationData, boolean, boolean> = useSWR(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation`, fetcher);


    useEffect(() => {
        if (data) {
            let filteredData = filteredDay ? data.reservations.filter(x => x.day_of_week_id == filteredDay) : data.reservations;
            filteredData = filteredDate ? filteredData.filter(x => x.date == filteredDate) : filteredData;
            filteredData = filteredLocation ? filteredData.filter(x => x.location_id == filteredLocation) : filteredData;
            filteredData = filteredTime ? filteredData.filter(x => x.start_time_id == filteredTime) : filteredData;
            let this_week_dates = [...new Map(filteredData.filter(x => x.next_week === false).map(x => [x.date, { key: x.date, value: `${x.date} (${x.day_of_week_name})` }])).values()];
            let next_week_dates = [...new Map(filteredData.filter(x => x.next_week === true).map(x => [x.date, { key: x.date, value: `${x.date} (${x.day_of_week_name})` }])).values()];
            let dates = [...new Map(filteredData.map(x => [x.date, { key: x.date, value: `${x.date} (${x.day_of_week_name})` }])).values()];
            let locations = [...new Map(filteredData.map(x => [x.location_id, { key: x.location_id, value: x.location_name }])).values()];
            let times = [...new Map(filteredData.map(x => [x.start_time_id, { key: x.start_time_id, value: x.start_time_name }])).values()]
            locations.sort((a, b) => sortThings(a.value, b.value));
            this_week_dates.sort((a, b) => sortThings(a.value, b.value));
            next_week_dates.sort((a, b) => sortThings(a.value, b.value));
            times.sort((a, b) => sortThings(a.key, b.key));
            setDates(dates);
            setThisWeekDates(this_week_dates);
            setNextWeekDates(next_week_dates);
            setLocations(locations);
            setStartTimes(times)
        }
    }, [data, filteredDate, filteredLocation, filteredDay, filteredTime]);


    useEffect(() => {
        if (data) {
            const milliseconds = (data.time_until_next_unlock * 1000) + 1000;
            setTimeout(() => {
                mutate(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation`);
            }, milliseconds)
        };
    }, [data])

    function sortThings(a: any, b: any): any {
        let left: any = a > b;
        let right: any = a < b;
        return left - right
    }
    useEffect(() => {
        if (dates.length == 1 && locations.length == 1 && startTimes.length == 1) {
            setIsReservable(true);
        } else {
            setIsReservable(false);
        }
    }, [dates, locations, startTimes])


    function resetFilters() {
        setFilteredDate(undefined);
        setFilteredLocation(undefined);
        setFilteredDay(undefined);
        setFilteredTime(undefined);
        setToggleThisWeekReset(true);
        setToggleNextWeekReset(true);
        setToggleLocationsReset(true);
        setToggleTimesReset(true);
    }

    async function handleReserve() {
        const allSelections: AllSelections = {
            "location": locations[0].key,
            "date": dates[0].key,
            "startTime": startTimes[0].key,
            "jwt": jwt
        };
        if (data) {
            const reservation_id = data.reservations.filter(x => x.location_id == allSelections.location && x.start_time_id == allSelections.startTime && x.date == allSelections.date)[0].reservation_id;

            await fetch(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${reservation_id}`, {
                method: "POST",
                headers: {
                    "authorization": `Bearer ${jwt}`
                },
            }).then(res => {
                if (res.status == 401) {
                    onOpen();
                    setModalHeader("Error")
                    setModalText("This session is no longer valid. Please log in again.")
                } else if (res.status == 402) {
                    onOpen();
                    setModalHeader("Error")
                    setModalText("You do not have enough booth picks left at this time.")
                    resetFilters()
                } else if (res.status == 409) {
                    onOpen();
                    setModalHeader("Error")
                    setModalText("Failure! Looks like someone got to this one right before you!")
                    mutate(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation`)
                    resetFilters()
                } else if (res.status == 200) {
                    onOpen();
                    setModalHeader("Success")
                    setModalText("Reserved!")

                    resetFilters()
                }
            })
        }
    }


    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>

    return (
        <>
            <h1><b>Dashboard Page</b></h1>
            <UserData />
            <Spacer y={4} />
            <Modal isOpen={isOpen} onOpenChange={onOpenChange} backdrop="blur">
                <ModalContent>
                    {(onClose) => (
                        <>
                            <ModalHeader className="flex flex-col gap-1">{modalHeader}</ModalHeader>
                            <ModalBody>
                                <p>
                                    {modalText}
                                </p>
                            </ModalBody>
                            <ModalFooter>
                                <Button color="primary" onPress={onClose}>
                                    Ok
                                </Button>
                            </ModalFooter>
                        </>
                    )}
                </ModalContent>
            </Modal>
            <div className="flex gap-2">
                <div >
                    <EndpointListbox label="This Week" toggleReset={toggleThisWeekReset} setToggleReset={setToggleThisWeekReset} setter={setFilteredDate} data={thisWeekDates} />
                    <EndpointListbox label="Next Week" toggleReset={toggleNextWeekReset} setToggleReset={setToggleNextWeekReset} setter={setFilteredDate} data={nextWeekDates} />
                </div>
                <EndpointListbox label="Locations" toggleReset={toggleLocationsReset} setToggleReset={setToggleLocationsReset} currentFilter={filteredDate} setter={setFilteredLocation} data={locations} />
                <EndpointListbox label="Time" toggleReset={toggleTimesReset} setToggleReset={setToggleTimesReset} currentFilter={filteredDate} setter={setFilteredTime} data={startTimes} />
            </div>
            <Spacer y={4} />
            <ReserveButton clickHandler={handleReserve} isDisabled={!isReservable} />
        </>
    )
}

function ReserveButton({ clickHandler, isDisabled }: any) {
    return (
        <Button color="primary" isDisabled={isDisabled} onPress={clickHandler}>Reserve</Button>
    )
}

function EndpointListbox({ label, toggleReset, setToggleReset, setter, data }: any) {
    const [selectedKeys, setSelectedKeys] = useState(new Set([]));

    useEffect(() => {
        if (toggleReset === true) {
            setSelectedKeys(new Set([]));
            setToggleReset(false)
        }
    }, [toggleReset, setToggleReset])


    function selectItem(key: any) {
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
                    selectionMode="single"
                    selectedKeys={selectedKeys}
                    onSelectionChange={selectItem}
                >
                    {(item: any) => (
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
