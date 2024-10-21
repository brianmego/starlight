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

export type Locations = [Location];

export type LockedData = {
    locations: [Location]
}
