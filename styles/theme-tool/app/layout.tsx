import * as React from 'react';

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
            <head />
            <body>{children}</body>
        </html>
    );
}
