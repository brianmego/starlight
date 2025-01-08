'use client';
import { useEffect, useState } from "react";
import useSWR, { SWRResponse, useSWRConfig } from 'swr';
import { getCookie } from 'cookies-next'
import { Button, Card, CardHeader, Divider, Link, Tabs, Tab } from "@nextui-org/react";
import { ReservationData, ReservationDataRow } from '@/app/lib/definitions';

const fetcher = (url: RequestInfo) => fetch(url).then(res => res.json());

export default function Page() {
    const { mutate } = useSWRConfig()
    let jwt = getCookie('jwt')?.toString()
    let id = ";"
    if (jwt === undefined) {
        id = ""
    } else {
        id = JSON.parse(atob(jwt.split('.')[1])).ID.split(':')[1]
    }
    const { data, error, isLoading }: SWRResponse<ReservationData, boolean, boolean> = useSWR(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${id}`, fetcher);
    const [nextWeekReservations, setNextWeekReservations] = useState(Array<ReservationDataRow>);
    const [freeReservations, setFreeReservations] = useState(Array<ReservationDataRow>);

    useEffect(() => {
        if (data) {
            setFreeReservations(data.filter(x => x.next_week == false))
            setNextWeekReservations(data.filter(x => x.next_week == true))
        }
    }, [data])

    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>

    async function deleteHandler(reservation_id: string) {
        console.log(reservation_id);
        await fetch(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${reservation_id}`, {
            method: "DELETE",
            headers: {
                "authorization": `Bearer ${jwt}`
            }
        }).then((x) => {
            if (x.status == 401) {
                {
                    alert("This session is no longer valid. Please log in again.")
                }
            }
        })

        mutate(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${id}`)
    }

    return <>
        <h1><b>My Reservations</b></h1>
        <div className="flex w-full flex-col">
            <Tabs aria-label="Options">
                <Tab key="free" title="Free Reservations">
                    {freeReservations.map(
                        (row, i) =>
                            <Card key={i} className="max-w-[400px]">
                                <CardHeader className="flex gap-3">
                                    <div className="flex flex-col">
                                        <p className="text-md">{row.date} ({row.day_of_week_name})</p>
                                        <p className="text-md">{row.location_name}</p>
                                        <p className="text-small text-default-500">{row.start_time_name}</p>
                                        {row.next_week === true &&
                                            <p className="text-small text-default-500">Worth a token!</p>
                                        }
                                        <Button color="primary" onPress={() => { deleteHandler(row.reservation_id) }}>Delete</Button>
                                    </div>
                                </CardHeader>
                                <Divider />
                            </Card>
                    )}
                </Tab>
                <Tab key="next" title="Next Week Reservations">
                    {nextWeekReservations.map(
                        (row, i) =>
                            <Card key={i} className="max-w-[400px]">
                                <CardHeader className="flex gap-3">
                                    <div className="flex flex-col">
                                        <p className="text-md">{row.date} ({row.day_of_week_name})</p>
                                        <p className="text-md">{row.location_name}</p>
                                        <p className="text-small text-default-500">{row.start_time_name}</p>
                                        {row.next_week === true &&
                                            <p className="text-small text-default-500">Worth a token!</p>
                                        }
                                        <Button color="primary" onPress={() => { deleteHandler(row.reservation_id) }}>Delete</Button>
                                    </div>
                                </CardHeader>
                                <Divider />
                            </Card>
                    )}
                </Tab>
            </Tabs>
            Don't forget to complete the google form about your booth: <Link isExternal showAnchorIcon href="https://www.google.com">Google Form</Link>
        </div>

    </>;
}
