use axum::response::Html;
use shtml::{html, Component, Render};

use crate::templates::attributes::Attrs::*;
use crate::templates::components::button::ButtonVarient;
use crate::templates::{
    components::{button::Button, Heading1},
    layout::RootLayout,
};

pub async fn root_page() -> Html<String> {
    let html_compnent = html! {
        <RootLayout>
          <Button props=vec![Class("text-5xl"),
                Varient(ButtonVarient::Danger),
                HxSwap("innerHTML"),
                HxTarget("#swapHere")
                ]>My Custom Button</Button>
          <button hx-post="/clicked" hx-swap="outerHTML">
              Click Me
          </button>
          <Heading1 class="p-6 border border-black text-red-500 text-5xl">Test</Heading1>
          <div id="swapHere"></div>
        </RootLayout>
    };
    Html(html_compnent.to_string())
}
