'use server';
import { serialize } from 'cookie';
import { signIn } from '@/auth';
import { AuthError } from 'next-auth';

export async function authenticate(
    prevState: string | undefined,
    formData: FormData,
) {
    try {
        let user = await signIn('credentials', formData);
        // serialize('session', "yourmom", {
        //     httpOnly: true,
        //     maxAge: 60 * 60, // one hour
        //     path: '/',
        // })
    } catch (error) {
        if (error instanceof AuthError) {
            switch (error.type) {
                case 'CredentialsSignin':
                    return 'Invalid Credentials.';
                default:
                    return 'Something went wrong.';
            }
        }
        throw error;
    }
}
