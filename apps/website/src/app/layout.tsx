import "../styles/index.css";

import { IconoirProvider } from "iconoir-react";
import LovedNavbar from "@/components/LovedNavbar";
import { Torus } from "@/fonts/torus";

export default function RootLayout({
    children
}: Readonly<{ children: React.ReactNode }>) {
    return (
        <html lang="en">
            <body className={`${Torus.className}`}>
                <div id="app" className="relative z-[1]">
                    <IconoirProvider
                        iconProps={{
                            strokeWidth: 2,
                            width: "1em",
                            height: "1em"
                        }}
                    >
                        <LovedNavbar />
                        {children}
                    </IconoirProvider>
                </div>
            </body>
        </html>
    );
}
