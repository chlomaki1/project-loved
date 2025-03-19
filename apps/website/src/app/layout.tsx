import "../styles/index.css";

export default function RootLayout({
    children
}: Readonly<{ children: React.ReactNode }>) {
    return (
        <html lang="en">
            <body>
                <div id="app">
                    {children}
                </div>
            </body>
        </html>
    );
}
