"use client";

import styles from "./item.module.css";
import { useContext, useEffect, useRef, useState } from "react";
import { Navbar } from ".";

interface ItemProps {
    href: string;
    children: React.ReactNode;
}

export function Item({ children, href }: ItemProps) {
    const elementRef = useRef<HTMLAnchorElement>(null);
    const context = useContext(Navbar.Context);
    const sectionContext = useContext(Navbar.Section.Context);
    const [isCurrent, setIsCurrent] = useState(false);

    useEffect(() => {
        setIsCurrent(context?.currentURI == href);
        if (isCurrent && sectionContext !== null) {
            sectionContext.currentElement = elementRef.current;
        }
    }, [isCurrent])

    return <a ref={elementRef} className={`relative ${isCurrent ? styles["glow-line"] : "opacity-50"}`} href={href}>{children}</a>;
}