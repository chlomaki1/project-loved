"use client";

import { createContext, useMemo } from "react";
import { usePathname } from "next/navigation";
import { Item } from "./item";
import { Section } from "./section";

interface NavbarContext {
    currentItem: string;
}

const NavbarContext = createContext<NavbarContext | null>(null);

interface NavbarProps {
    children: React.ReactNode;
}

export function Navbar({ children }: NavbarProps) {
    const uri = usePathname();
    const context = useMemo(() => ({ currentItem: uri }), [uri]);

    return <div className="flex flex-row gap-4 items-center h-20 p-2 bg-[#301D25]">
        <NavbarContext.Provider value={context}>
            {children}
        </NavbarContext.Provider>
    </div>;
}

Navbar.Item = Item;
Navbar.Section = Section;
Navbar.Context = NavbarContext;
