#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

pub fn RootLayout(children: Elements) -> Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>Portfolio</title>
                <link href="/css/output.css" rel="stylesheet" />
            </head>
            <body>
                {children}
            </body>
        </html>
    }
}
