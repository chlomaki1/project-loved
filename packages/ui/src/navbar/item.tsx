"use client";

import { useContext, useMemo } from "react";
import { Navbar } from ".";

interface ItemProps {
    href: string;
    children: React.ReactNode;
}

export function Item({ children, href }: ItemProps) {
    const context = useContext(Navbar.Context);
    const isCurrent = useMemo(() => context?.currentItem == href, [context, href]);

    return <a className={`${isCurrent ? "" : "opacity-50"}`} href={href}>{children}</a>;
}