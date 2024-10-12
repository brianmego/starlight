'use server';
import { Locations } from '@/app/lib/definitions';

export async function getLocations(): Promise<Locations> {
    const res = await fetch("http://localhost:1912/api/location", {
        method: "GET",
        headers: {
            "content-type": "application/json"
        }
    });
    if (res.ok) {
        return await res.json();
    } else {
        throw new Error('No locations returned')
    }
}

