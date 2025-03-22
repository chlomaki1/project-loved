"use client";

import { createContext, useEffect, useMemo, useState } from "react";
import { usePathname } from "next/navigation";
import { Item } from "./item";
import { Section } from "./section";
import { motion, scroll } from "motion/react";

interface NavbarContext {
    currentURI: string;
}

const NavbarContext = createContext<NavbarContext | null>(null);

interface NavbarProps {
    children: React.ReactNode;
}

export function Navbar({ children }: NavbarProps) {
    const [scrolled, setScrolled] = useState(false);
    const uri = usePathname();
    const context = useMemo(
        () => ({ currentURI: uri } satisfies NavbarContext),
        [uri]
    );

    useEffect(() => {
        scroll((p: number) => {
            if (p > 0) {
                setScrolled(true);
            } else {
                setScrolled(false);
            }
        })
    }, []);

    return <div className="sticky top-0 flex flex-row gap-4 items-center h-20 p-2">
        <motion.div
            animate={{
                opacity: scrolled ? 0 : 1,
                height: scrolled ? "calc(var(--spacing) * 20)" : "50vh"
            }}
            transition={{
                duration: 0.25,
                type: "tween",
                ease: "easeOut"
            }}
            className="pointer-events-none absolute inset-0 h-[50vh] -z-50 bg-linear-to-b from-[#FF338F0F] to-[#FF338F00]"
        />
        <motion.div
            animate={{
                opacity: scrolled ? 1 : 0,
            }}
            transition={{
                duration: 0.25,
                type: "tween",
                ease: "easeOut"
            }}
            className="pointer-events-none absolute inset-0 -z-50 bg-[#301D25] opacity-0"
        />
        <NavbarContext.Provider value={context}>
            {children}
        </NavbarContext.Provider>
    </div>;
}

Navbar.Item = Item;
Navbar.Section = Section;
Navbar.Context = NavbarContext;
