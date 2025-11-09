'use client';
import { useEffect, useState } from "react";
import Dashboard from "../../dashboard/page";
import { ModeContext, RenderMode } from "../../dashboard/page";
import useSWR, { SWRResponse, useSWRConfig } from 'swr';
import { getCookie } from 'cookies-next'
import { Button, Card, CardHeader, Divider, Link, Tabs, Tab, useDisclosure, Modal, ModalContent, ModalHeader, ModalBody, ModalFooter, Spacer } from "@heroui/react";
import { Drawer, DrawerContent, DrawerHeader, DrawerBody, DrawerFooter } from "@heroui/react";
import { UserReservationData, ReservationDataRow } from '@/app/lib/definitions';

enum Action {
    Swap,
    Delete
}

const fetcher = (url: RequestInfo) => fetch(url).then(res => res.json());

export default function Page() {
    const { mutate } = useSWRConfig()
    let jwt = getCookie('jwt')?.toString()
    const failureModal = useDisclosure();
    const confirmationModal = useDisclosure();
    const swapDrawer = useDisclosure();
    const [modalText, setModalText] = useState("");
    const [modalHeader, setModalHeader] = useState("");
    const [selectedReservationId, setSelectedReservationId] = useState("");
    const [action, setAction] = useState(Action.Delete);

    let id = "";
    if (jwt === undefined) {
        id = ""
    } else {
        id = JSON.parse(atob(jwt.split('.')[1])).ID.split(':')[1]
    }
    const { data, error, isLoading }: SWRResponse<UserReservationData, boolean, boolean> = useSWR(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${id}`, fetcher);
    const [nextWeekReservations, setNextWeekReservations] = useState(Array<ReservationDataRow>);
    const [thisWeekReservations, setThisWeekReservations] = useState(Array<ReservationDataRow>);
    const [previousReservations, setPreviousReservations] = useState(Array<ReservationDataRow>);

    useEffect(() => {
        if (data) {
            setPreviousReservations(data.filter(x => x.passed == true))
            setThisWeekReservations(data.filter(x => (x.next_week == false && x.passed == false)))
            setNextWeekReservations(data.filter(x => x.next_week == true))
        }
    }, [data])

    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>

    async function showDeleteConfirmation(reservation_id: string) {
        setSelectedReservationId(reservation_id);
        confirmationModal.onOpen();
        setModalHeader("Confirm delete");
        setModalText("Are you sure? Giving this up means others will be allowed to reserve this booth.");
        setAction(Action.Delete);
    }

    async function showSwap(reservation_id: string) {
        setSelectedReservationId(reservation_id);
        swapDrawer.onOpen();
        setAction(Action.Swap);
    }

    async function deleteHandler() {
        await fetch(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${selectedReservationId}`, {
            method: "DELETE",
            headers: {
                "authorization": `Bearer ${jwt}`
            }
        }).then((x) => {
            if (x.status == 401) {
                {
                    failureModal.onOpen();
                    setModalHeader("Error")
                    setModalText("This session is no longer valid. Please log in again.")
                }
            }
        })

        mutate(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${id}`)
        confirmationModal.onClose();
    }

    async function swapHandler() {
        await fetch(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/reserveswap/${selectedReservationId}`, {
            method: "POST",
            headers: {
                "authorization": `Bearer ${jwt}`
            }
        }).then((x) => {
            if (x.status == 401) {
                {
                    failureModal.onOpen();
                    setModalHeader("Error")
                    setModalText("This session is no longer valid. Please log in again.")
                }
            }
        })

        mutate(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${id}`)
        confirmationModal.onClose();
    }

    function closeSwap() {
        swapDrawer.onClose();
        mutate(`${process.env.NEXT_PUBLIC_API_ROOT}/reservation/${id}`)

    }

    return <>
        <h1><b>My Reservations</b></h1>
        <Drawer isOpen={swapDrawer.isOpen} onOpenChange={swapDrawer.onOpenChange} size="2xl" placement="bottom">
            <ModeContext.Provider value={{renderMode: RenderMode.Swap, params: {oldId: selectedReservationId, closeCallback: closeSwap}}}>
                <DrawerContent>
                    {(onClose) => (
                        <>
                            <DrawerHeader className="flex flex-col gap-1">Choose new reservation</DrawerHeader>
                            <DrawerBody>
                                <Dashboard />
                            </DrawerBody>
                            <DrawerFooter>
                                <Button onPress={onClose}>
                                    Close
                                </Button>
                            </DrawerFooter>
                        </>
                    )}
                </DrawerContent>
            </ModeContext.Provider>
        </Drawer>
        <Modal isOpen={confirmationModal.isOpen} onOpenChange={confirmationModal.onOpenChange} backdrop="blur">
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
                            <Button color="default" onPress={onClose}>
                                Close
                            </Button>
                            <Button color="primary" onPress={action == Action.Delete ? deleteHandler : swapHandler}>
                                Confirm
                            </Button>
                        </ModalFooter>
                    </>
                )}
            </ModalContent>
        </Modal>
        <Modal isOpen={failureModal.isOpen} onOpenChange={failureModal.onOpenChange} backdrop="blur">
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
                <Tab key="thisweek" title="This Week Reservations">
                    <ThisWeekReservations reservations={thisWeekReservations} showDeleteConfirmation={showDeleteConfirmation} showSwap={showSwap} />
                </Tab>
                <Tab key="next" title="Next Week Reservations">
                    <NextWeekReservations reservations={nextWeekReservations} showDeleteConfirmation={showDeleteConfirmation} showSwap={showSwap} />
                </Tab>
                <Tab key="previous" title="Previous Reservations">
                    <PreviousReservations reservations={previousReservations} />
                    <p>
                        Don&apos;t forget to complete the google form about your booth: <Link isExternal showAnchorIcon href="https://docs.google.com/forms/d/e/1FAIpQLSflzxS_c2HTWysCg2ICEBCDt7YON_-kzw_WqajMA79n0v5NRg/viewform">Google Form</Link>
                    </p>
                </Tab>
            </Tabs>
        </div>

    </>;
}

function ThisWeekReservations({ reservations, showDeleteConfirmation, showSwap }: {
    reservations: ReservationDataRow[],
    showDeleteConfirmation: any,
    showSwap: any
}) {
    return (
        <>
            {reservations.map(
                (row, i) =>
                    <Card key={i} className="max-w-[400px]">
                        <CardHeader className="flex gap-3">
                            <div className="flex flex-col">
                                <p className="text-md">{row.date} ({row.day_of_week_name})</p>
                                <p className="text-md">{row.location_name}</p>
                                <Spacer y={2} />
                                <p className="text-md text-default-500">Address: {row.location_address}</p>
                                <p className="text-small text-default-500">Time: {row.start_time_name}</p>
                                <Spacer y={2} />
                                {row.location_notes &&
                                    <>
                                        <p className="text-small text-default-500">Notes: {row.location_notes}</p>
                                        <Spacer y={2} />
                                    </>
                                }
                                <Spacer y={2} />
                                <Button color="primary" onPress={() => { showDeleteConfirmation(row.reservation_id) }}>Delete</Button>
                                <Spacer y={2} />
                                <Button color="primary" onPress={() => { showSwap(row.reservation_id) }}>Swap</Button>
                            </div>
                        </CardHeader>
                        <Divider />
                    </Card>
            )}
        </>
    )

}
function NextWeekReservations({ reservations, showDeleteConfirmation, showSwap }: {
    reservations: ReservationDataRow[],
    showDeleteConfirmation: any,
    showSwap: any
}) {

    return (
        <>
            <p>You get booth picks back if you give one of these up</p>
            <Spacer y={2} />
            {reservations.map(
                (row, i) =>
                    <div key={i}>
                        <Card key={i} className="max-w-[400px]">
                            <CardHeader className="flex gap-3">
                                <div className="flex flex-col">
                                    <p className="text-md">{row.date} ({row.day_of_week_name})</p>
                                    <p className="text-md">{row.location_name}</p>
                                    <Spacer y={2} />
                                    <p className="text-md text-default-500">Address: {row.location_address}</p>
                                    <p className="text-small text-default-500">Time: {row.start_time_name}</p>
                                    <Spacer y={2} />
                                    {row.location_notes &&
                                        <>
                                            <p className="text-small text-default-500">Notes: {row.location_notes}</p>
                                            <Spacer y={2} />
                                        </>
                                    }
                                    <Spacer y={2} />
                                    <Button color="primary" onPress={() => showDeleteConfirmation(row.reservation_id)}>Delete</Button>
                                    <Spacer y={2} />
                                    <Button color="primary" onPress={() => { showSwap(row.reservation_id) }}>Swap</Button>
                                </div>
                            </CardHeader>
                            <Divider />
                        </Card>
                        <Spacer y={2} />
                    </div>
            )}
        </>
    )
}

function PreviousReservations({ reservations }: { reservations: ReservationDataRow[] }) {
    return (
        <>
            <Spacer y={2} />
            {reservations.map(
                (row, i) =>
                    <div key={i}>
                        <Card key={i} className="max-w-[400px]">
                            <CardHeader className="flex gap-3">
                                <div className="flex flex-col">
                                    <p className="text-md">{row.date} ({row.day_of_week_name})</p>
                                    <p className="text-md">{row.location_name}</p>
                                    <Spacer y={2} />
                                    <p className="text-md text-default-500">Address: {row.location_address}</p>
                                    <p className="text-small text-default-500">Time: {row.start_time_name}</p>
                                </div>
                            </CardHeader>
                            <Divider />
                        </Card>
                        <Spacer y={2} />
                    </div>
            )}
        </>
    )
}

