'use client';

import {
    BookOpenIcon,
    UserGroupIcon,
    HomeIcon,
} from '@heroicons/react/24/outline';
import Link from 'next/link';
import { getCookie } from 'cookies-next'
import { usePathname } from 'next/navigation';
import clsx from 'clsx';


export default function NavLinks() {
    const pathname = usePathname();
    let jwt = getCookie('jwt')?.toString()
    let is_admin = false
    if (jwt === undefined) {
        is_admin = false
    } else {
        is_admin = JSON.parse(atob(jwt.split('.')[1])).is_admin
    }
    let links = [
        { name: 'Dashboard', href: '/dashboard', icon: HomeIcon },
        { name: 'My Reservations', href: '/dashboard/reservations', icon: UserGroupIcon },
    ]
    if (is_admin === true) {
        links.push(
            { name: 'Admin', href: '/dashboard/admin', icon: BookOpenIcon }

        )
    };
    return (
        <>
            {links.map((link) => {
                const LinkIcon = link.icon;
                return (
                    <Link
                        key={link.name}
                        href={link.href}
                        className={clsx(
                            "flex h-[48px] grow items-center justify-center gap-2 rounded-md bg-gray-50 p-3 text-sm font-medium hover:bg-sky-100 hover:text-blue-600 md:flex-none md:jrstify-start md:p-2 md:px-3",
                            {
                                'bg-sky-100 text-blue-600': pathname === link.href,
                            }
                        )}
                    >
                        <LinkIcon className="w-6" />
                        <p className="hidden md:block">{link.name}</p>
                    </Link>
                );
            })}
        </>
    );
}
