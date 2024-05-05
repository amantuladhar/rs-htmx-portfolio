#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

use crate::templates::{
    attributes::Attrs::*,
    components::button::{Button, ButtonVarient::*},
};

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
                <NavBar/>
                <main id="main-body" class="min-w-[100dvh] p-6">
                    {children}
                </main>
            </body>
        </html>
    }
}

pub fn NavBar() -> Component {
    html! {
        <nav class="bg-white shadow-[0_1px_0px_1px_black] px-2 py-2">
            <ul class="flex flex-row gap-2">
                <NavItem path="/home">Home</NavItem>
                <NavItem path="/about">About</NavItem>
            </ul>
        </nav>
    }
}

pub fn NavItem(path: &str, children: Elements) -> Component {
    html! {
        <li>
            <Button props=vec![
                        Varient(Ghost),
                        HxSwap("innerHTML"),
                        HxTarget("#main-body"),
                        HxGet(path)]>
                {children}
            </Button>
        </li>
    }
}
