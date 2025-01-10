'use client';
import { useEffect, useState } from "react";
import useSWR, { SWRResponse, useSWRConfig } from 'swr';
import { getCookie } from 'cookies-next'
import { Button, Card, CardHeader, Divider, Link, Tabs, Tab, useDisclosure, Modal, ModalContent, ModalHeader, ModalBody, ModalFooter } from "@nextui-org/react";
import { UserReservationData, ReservationDataRow } from '@/app/lib/definitions';

const fetcher = (url: RequestInfo) => fetch(url).then(res => res.json());

export default function Page() {
    const { mutate } = useSWRConfig()
    let jwt = getCookie('jwt')?.toString()
    const { isOpen, onOpen, onOpenChange } = useDisclosure();
    const [modalText, setModalText] = useState("");
    const [modalHeader, setModalHeader] = useState("");

    let id = ";"
    if (jwt === undefined) {
        id = ""
    } else {
        id = JSON.parse(atob(jwt.split('.')[1])).ID.split(':')[1]
    }
    const { data, error, isLoading }: SWRResponse<UserReservationData, boolean, boolean> = useSWR(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${id}`, fetcher);
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
                    onOpen();
                    setModalHeader("Error")
                    setModalText("This session is no longer valid. Please log in again.")
                }
            }
        })

        mutate(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${id}`)
    }

    return <>
        <h1><b>My Reservations</b></h1>
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
            <p>
                Don&apos;t forget to complete the google form about your booth: <Link isExternal showAnchorIcon href="https://www.google.com">Google Form</Link>
            </p>
        </div>

    </>;
}
