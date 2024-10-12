export type AuthenticatedUser = {
    username: string;
    jwt: string;
};

export type Location = {
    name: string;
}

export type Locations = [Location];
