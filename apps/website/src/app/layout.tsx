import "../styles/index.css";

import { IconoirProvider } from "iconoir-react";
import { Navbar } from "@loved/ui";
import { Torus } from "@/fonts/torus";

export default function RootLayout({
    children
}: Readonly<{ children: React.ReactNode }>) {
    return (
        <html lang="en">
            <body className={`${Torus.className}`}>
                <div id="app">
                    <IconoirProvider
                        iconProps={{
                            strokeWidth: 2,
                            width: "1em",
                            height: "1em"
                        }}
                    >
                        <Navbar />
                        {children}
                    </IconoirProvider>
                </div>
            </body>
        </html>
    );
}
