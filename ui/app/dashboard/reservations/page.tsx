'use client';
import { useRouter } from 'next/navigation';
import useSWR, { useSWRConfig } from 'swr';
import { getCookie } from 'cookies-next'
import jwt_decode from 'jwt-decode'
import { Button, Card, CardHeader, CardBody, Divider } from "@nextui-org/react";
import { redirect } from 'next/dist/server/api-utils';

export default function Page() {
    const router = useRouter()
    const { mutate } = useSWRConfig()
    const jwt = getCookie('jwt')?.toString()
    const id = JSON.parse(atob(jwt.split('.')[1])).ID.split(':')[1]
    const fetcher = (...args) => fetch(...args).then(res => res.json());
    const { data, error, isLoading } = useSWR(`http://0:1912/api/reservation/${id}`, fetcher);
    //
    if (error) return <p>failed to load</p>
    if (isLoading) return <p>Loading...</p>

    async function deleteHandler(reservation_id: string) {
        console.log(reservation_id);
        await fetch(`http://0:1912/api/reservation/${reservation_id}`, {
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

        mutate(`http://0:1912/api/reservation/${id}`)
    }

    return <>
        <h1><b>My Reservations</b></h1>
        {data.map(
            (row, i) =>
                <Card key={i} className="max-w-[400px]">
                    <CardHeader className="flex gap-3">
                        <div className="flex flex-col">
                            <p className="text-md">{row.location_name} - {row.day_of_week_name}</p>
                            <p className="text-small text-default-500">{row.start_time_name}</p>
                            <Button color="primary" onPress={() => { deleteHandler(row.reservation_id) }}>Delete</Button>
                        </div>
                    </CardHeader>
                    <Divider />
                </Card>
        )}

    </>;
}
