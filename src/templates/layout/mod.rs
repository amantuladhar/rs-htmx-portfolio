#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

pub fn RootLayout(children: Elements) -> Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>Portfolio</title>
                <link href="/css/output.css" rel="stylesheet" />
                <script defer src="https://unpkg.com/htmx.org@1.9.12"></script>
            </head>
            <body>
                <main class="min-w-[100dvh] p-6">
                    {children}
                </main>
            </body>
        </html>
    }
}
