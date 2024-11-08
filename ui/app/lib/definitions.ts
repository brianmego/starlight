export type AuthenticatedUser = {
    username: string;
    jwt: string;
};

export type CredentialsInputs = {
    user: string;
    password: string;
}

export type Location = {
    name: string;
}
export type Day = {
    name: string
}
export type Timeslot = {
    name: string
}

export type Locations = [Location];

export type LockedData = {
    locations: [Location],
    days: [Day],
    timeslots: [Timeslot]
}

export type AllSelections = {
    location?: string,
    day?: string,
    timeslot?: string
}
