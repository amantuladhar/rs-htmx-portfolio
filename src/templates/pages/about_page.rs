use axum::response::Html;
use shtml::{html, Component, Render};

use crate::templates::layout::RootLayout;

pub async fn about_page() -> Html<String> {
    let html_compnent = html! {
        <RootLayout props=[]>
            <h1>About Page</h1>
        </RootLayout>
    };
    Html(html_compnent.to_string())
}
