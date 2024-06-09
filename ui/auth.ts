import NextAuth from 'next-auth';
import { authConfig } from './auth.config';
import Credentials from 'next-auth/providers/credentials';
import { AuthenticatedUser, CredentialsInputs } from '@/app/lib/definitions';

async function getUser(credentials): Promise<AuthenticatedUser | undefined> {
    const res = await fetch("http://localhost:1912/login", {
        method: "POST",
        body: JSON.stringify({
            user: credentials.user,
            password: credentials.password,
        }),
        headers: {
            "content-type": "application/json"
        }
    });
    if (res.ok) {
        // console.log((await res.json()).jwt);
        let jwt = (await res.json()).jwt;
        let user = { username: credentials.user, jwt: jwt };
        return user;
    } else {
        throw new Error('Bad credentials');
    }
}

export const { auth, signIn, signOut } = NextAuth({
    ...authConfig,
    providers: [Credentials({
        async authorize(credentials) {
            // async function login(formData: FormData) {
            'use server'
            let user = await getUser(credentials);
            if (user) {
                return user;
            } else {
                return null;
            }
        }
    })],
});

