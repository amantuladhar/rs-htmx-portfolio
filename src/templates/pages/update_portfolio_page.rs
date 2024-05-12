use axum::response::Html;
use shtml::{html, Component, Render};

use crate::templates::layout::RootLayout;

pub async fn update_portfolio_page() -> Html<String> {
    let page = html! {
    <RootLayout props=[]>
        <h1>Update Portfolio</h1>
    </RootLayout>
    };
    Html(page.to_string())
}
