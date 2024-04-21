import { Inter, Lusitana, Happy_Monkey } from 'next/font/google';
export const happy_monkey = Happy_Monkey({ subsets: ['latin'], weight: ['400'] });
export const inter = Inter({ subsets: ['latin'] });
export const lusitana = Lusitana({
    subsets: ['latin'], weight: ['400', '700'],
});
