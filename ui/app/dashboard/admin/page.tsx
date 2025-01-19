'use client';
import { useState } from "react";
import { getCookie } from 'cookies-next'
import { AsyncListData, useAsyncList } from "@react-stately/data";
import {
    Table,
    TableHeader,
    TableColumn,
    TableBody,
    TableRow,
    TableCell,
    getKeyValue,
    Spinner,
} from "@nextui-org/react";
import { CurrentReservationDataRow } from "@/app/lib/definitions";


export default function Page() {
    let jwt = getCookie('jwt')?.toString()

    let id = ";"
    if (jwt === undefined) {
        id = ""
    } else {
        id = JSON.parse(atob(jwt.split('.')[1])).ID.split(':')[1]
    }
    const [isLoading, setIsLoading] = useState(true);

    let list: AsyncListData<CurrentReservationDataRow> = useAsyncList({
        async load({ signal }) {
            let res = await fetch(`${process.env.NEXT_PUBLIC_API_ROOT}/history`, {
                signal,
            });
            let json = await res.json();

            setIsLoading(false);

            return {
                items: json,
            };
        },
        async sort({ items, sortDescriptor }) {
            return {
                items: items.sort((a: any, b: any) => {
                    let first = a[sortDescriptor.column];
                    let second = b[sortDescriptor.column];
                    let cmp = (parseInt(first) || first) < (parseInt(second) || second) ? -1 : 1;

                    if (sortDescriptor.direction === "descending") {
                        cmp *= -1;
                    }

                    return cmp;
                }),
            };
        },
    });

    return <>
        <h1><b>Reservations</b></h1>
        <Table
            isStriped
            aria-label="Table of Reservation Data"
            sortDescriptor={list.sortDescriptor}
            onSortChange={list.sort}
        >
            <TableHeader>
                <TableColumn key="date" allowsSorting>Date</TableColumn>
                <TableColumn key="time" allowsSorting>Time</TableColumn>
                <TableColumn key="location" allowsSorting>Location</TableColumn>
                <TableColumn key="username" allowsSorting>User</TableColumn>
            </TableHeader>
            <TableBody
                isLoading={isLoading}
                items={list.items}
                loadingContent={<Spinner label="Loading..." />}
            >
                {(item) => (
                    <TableRow key={item.id}>
                        {(columnKey) => <TableCell>{getKeyValue(item, columnKey)}</TableCell>}
                    </TableRow>
                )}
            </TableBody>
        </Table>

    </>;
}
