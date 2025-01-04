import NextAuth, { User } from 'next-auth';
import { authConfig } from './auth.config';
import Credentials from 'next-auth/providers/credentials';
import { AuthenticatedUser, CredentialsInputs } from '@/app/lib/definitions';
import { cookies } from 'next/headers'

declare module "next-auth" {
    interface User {
        jwt: string;
        username: string;
    }
}
async function getUser(credentials: CredentialsInputs): Promise<AuthenticatedUser> {
    const res = await fetch("http://0:1912/login", {
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
    providers: [
        Credentials({
            async authorize(credentials: any) {
                'use server'
                let authorized_user = await getUser(credentials);
                console.log(`User: ${authorized_user.jwt}`);
                cookies().set('jwt', authorized_user.jwt)
                if (authorized_user) {
                    const user: User = { jwt: "user", username: "username" };
                    return user
                } else
                    throw new Error("Login failed");
            }
        })],
});

