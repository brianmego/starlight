declare global {
    interface Window { ENV: any; }
}
declare global {
    namespace NodeJS {
        interface ProcessEnv {
            API_ROOT: string;
            AUTH_SECRET: string;
            LOGIN_URL: string;
        }
    }
}
export {}
