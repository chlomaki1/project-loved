import "../styles/index.css";

import { IconoirProvider } from "iconoir-react";
import { Navbar } from "@loved/ui";

export default function RootLayout({
    children
}: Readonly<{ children: React.ReactNode }>) {
    return (
        <html lang="en">
            <body>
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
