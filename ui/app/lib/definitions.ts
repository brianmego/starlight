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
    name: string;
}
export type ResDay = {
    key: number;
    name: string;
}
export type ResTime = {
    key: number
    name: number
}

export type Locations = [ResLocation];

export type LockedData = {
    locations: [ResLocation],
    days: [ResDay],
    startTime: [ResTime]
}

export type AllSelections = {
    location?: string;
    day?: string;
    startTime?: string;
    jwt?: string;
}
