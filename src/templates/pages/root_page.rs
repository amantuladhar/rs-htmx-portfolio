use axum::response::Html;
use shtml::{html, Component, Render};

use crate::templates::attributes::Attrs::*;
use crate::templates::components::button::ButtonVarient;
use crate::templates::{components::button::Button, layout::RootLayout};

pub async fn root_page() -> Html<String> {
    let html_compnent = html! {
        <RootLayout>
              <Button props=vec![
                        Class("text-5xl"),
                        Varient(ButtonVarient::Danger),
                        HxSwap("innerHTML"),
                        HxTarget("#swapHere")
              ]>My Custom Button</Button>
        </RootLayout>
    };
    Html(html_compnent.to_string())
}
