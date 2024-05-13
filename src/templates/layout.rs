#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

use crate::{
    auth::cookies_and_jwt::LoggedInUser,
    templates::{attributes::Attrs::*, components::link::Link},
};

use super::attributes::Attrs;

pub struct RootLayoutProps<'a> {
    logged_in_user: &'a Option<LoggedInUser>,
}

pub fn RootLayout(props: impl IntoRootLayoutProps, children: Elements) -> Component {
    let props = props.into_props();
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>Portfolio</title>
                <link href="/public/css/styles.css" rel="stylesheet" />
                <script defer src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.12/htmx.min.js" integrity="sha512-JvpjarJlOl4sW26MnEb3IdSAcGdeTeOaAlu2gUZtfFrRgnChdzELOZKl0mN6ZvI0X+xiX5UMvxjK2Rx2z/fliw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
                <script defer src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.12/ext/response-targets.min.js" integrity="sha512-l6UYjgNtXt4g4Lnl9nr6B54guXLkZLzmXjpO39jZR4ue/xv/O8IpU1HEvUWvE7Z0ZOsigu+2v7/2ldL6J5IljA==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
            </head>
            // Does hx-ext resonse-targets need to be on immediate parent? Looks like not
            <body hx-ext="response-targets">
                <NavBar props=[LoggedInUser(props.logged_in_user)]/>
                <main id="main-body" class="p-6">
                    {children}
                <section id="presentation-body"></section>
                </main>
            </body>
        </html>
    }
}

pub struct NavBarProps<'a> {
    logged_in_user: &'a Option<LoggedInUser>,
}

pub fn NavBar(props: impl IntoNavBarProps) -> Component {
    let props = props.into_props();
    html! {
        <nav class="sticky top-0 left-0 w-[100dvw] flex flex-row justify-between bg-white shadow-[0_0px_2px_1px_#bbbbbb] px-2 py-3 [&>*]:flex [&>*]:flex-row] [&>*]:gap-2">
            <ul class="left-nav">
                <NavItem path="/">Home</NavItem>
            </ul>
            <ul class="right-nav">
                {
                    props.logged_in_user.as_ref().map(|user| {
                        html! {
                            <NavItem path="/p/test">View</NavItem>
                            <NavItem path="/logout">Logout</NavItem>
                            <li class="flex justify-center items-center">
                                <div class="font-bold">{format!("Hello {}", user.username)}!</div>
                            </li>
                        }
                    })
                    .unwrap_or_else(|| {
                        html! {
                            <NavItem path="/login">Login</NavItem>
                        }
                    })
                }
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

pub trait IntoRootLayoutProps {
    fn into_props(&self) -> RootLayoutProps;
}

impl<'a, const SIZE: usize> IntoRootLayoutProps for [Attrs<'a, ()>; SIZE] {
    fn into_props(&self) -> RootLayoutProps {
        let mut props = RootLayoutProps {
            logged_in_user: &None,
        };
        self.iter().for_each(|attr| match *attr {
            Attrs::LoggedInUser(value) => props.logged_in_user = value,
            #[allow(unreachable_patterns)]
            _ => {}
        });
        props
    }
}

pub trait IntoNavBarProps {
    fn into_props(&self) -> NavBarProps;
}

impl<'a, const SIZE: usize> IntoNavBarProps for [Attrs<'a, ()>; SIZE] {
    fn into_props(&self) -> NavBarProps {
        let mut props = NavBarProps {
            logged_in_user: &None,
        };
        self.iter().for_each(|attr| match *attr {
            Attrs::LoggedInUser(value) => props.logged_in_user = value,
            #[allow(unreachable_patterns)]
            _ => {}
        });
        props
    }
}
