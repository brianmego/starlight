'use client';
import { useEffect, useState } from "react";
import useSWR, { SWRResponse, useSWRConfig } from 'swr';
import { getCookie } from 'cookies-next'
import {
    Table,
    TableHeader,
    TableColumn,
    TableBody,
    TableRow,
    TableCell,
    getKeyValue,
} from "@nextui-org/react";
import { ReservationLogData, ReservationLogDataRow } from '@/app/lib/definitions';


const fetcher = (url: RequestInfo) => fetch(url).then(res => res.json());

export default function Page() {
    let jwt = getCookie('jwt')?.toString()
    const rows = [
        {
            key: "1",
            name: "Tony Reichert",
            role: "CEO",
            status: "Active",
        },
        {
            key: "2",
            name: "Zoey Lang",
            role: "Technical Lead",
            status: "Paused",
        },
        {
            key: "3",
            name: "Jane Fisher",
            role: "Senior Developer",
            status: "Active",
        },
        {
            key: "4",
            name: "William Howard",
            role: "Community Manager",
            status: "Vacation",
        },
    ];

    const columns = [
        {
            key: "at",
            label: "Time",
        },
        {
            key: "action",
            label: "ACTION",
        },
    ];


    let id = ";"
    if (jwt === undefined) {
        id = ""
    } else {
        id = JSON.parse(atob(jwt.split('.')[1])).ID.split(':')[1]
    }
    const { data, error, isLoading }: SWRResponse<ReservationLogData, boolean, boolean> = useSWR(`${process.env.NEXT_PUBLIC_API_ROOT}/history`, fetcher);

    // useEffect(() => {
    //     if (data) {
    //     }
    // }, [data])

    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>

    return <>
        <h1><b>History</b></h1>
        <Table aria-label="Example table with dynamic content">
            <TableHeader columns={columns}>
                {(column) => <TableColumn key={column.key}>{column.label}</TableColumn>}
            </TableHeader>
            <TableBody items={data}>
                {(item) => (
                    <TableRow key={item.at}>
                        {(columnKey) => <TableCell>{getKeyValue(item, columnKey)}</TableCell>}
                    </TableRow>
                )}
            </TableBody>
        </Table>

    </>;
}
