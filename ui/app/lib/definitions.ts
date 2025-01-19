export type AuthenticatedUser = {
    username: string;
    jwt: string;
};

export type CredentialsInputs = {
    user: string;
    password: string;
}

export type ResLocation = {
    key: string;
    value: string;
}
export type ResDate = {
    key: string;
    value: string;
}
export type ResTime = {
    key: number;
    value: string;
}

export type Locations = [ResLocation];


export type AllSelections = {
    location: string;
    date: string;
    startTime: number;
    jwt?: string;
}

export type ReservationData = {
    time_until_next_unlock: number;
    reservations: [ReservationDataRow];
};

export type ReservationDataRow = {
    reservation_id: string;
    date: string;
    day_of_week_id: number;
    day_of_week_name: string;
    location_id: string;
    location_name: string;
    location_address: string;
    location_notes?: string;
    start_time_id: number;
    start_time_name: string;
    next_week: boolean;
    passed: boolean;
}

export type UserReservationData = [ReservationDataRow];

export type CurrentReservationDataRow = {
    id: string;
    date: string;
    username: string;
    location: string;
}

export type CurrentReservationData = [CurrentReservationDataRow];
