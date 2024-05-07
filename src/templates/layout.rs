#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

use crate::templates::{attributes::Attrs::*, components::link::Link};

pub fn RootLayout(children: Elements) -> Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>Portfolio</title>
                <link href="/public/css/styles.css" rel="stylesheet" />
                <script defer src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.12/htmx.min.js" integrity="sha512-JvpjarJlOl4sW26MnEb3IdSAcGdeTeOaAlu2gUZtfFrRgnChdzELOZKl0mN6ZvI0X+xiX5UMvxjK2Rx2z/fliw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
                <script defer src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.12/ext/response-targets.min.js" integrity="sha512-l6UYjgNtXt4g4Lnl9nr6B54guXLkZLzmXjpO39jZR4ue/xv/O8IpU1HEvUWvE7Z0ZOsigu+2v7/2ldL6J5IljA==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
            </head>
            <body>
                <NavBar/>
                <main id="main-body" class="p-6">
                    {children}
                </main>
            </body>
        </html>
    }
}

pub fn NavBar() -> Component {
    html! {
        <nav class="flex flex-row justify-between bg-white shadow-[0_1px_0px_1px_black] px-2 py-3 [&>*]:flex [&>*]:flex-row] [&>*]:gap-2">
            <ul class="left-nav">
                <NavItem path="/">Home</NavItem>
                <NavItem path="/about">About</NavItem>
            </ul>
            <ul class="right-nav">
                <NavItem path="/login">Login</NavItem>
            </ul>
        </nav>
    }
}

pub fn NavItem(path: &str, children: Elements) -> Component {
    html! {
        <li>
            <Link props=[Class("block"),
                        HxSwap("outerHTML transition:true"),
                        HxTarget("#main-body"),
                        HxSelect("#main-body"),
                        HxPushUrl("true"),
                        HxGet(path)]>
                {children}
            </Link>
        </li>
    }
}
