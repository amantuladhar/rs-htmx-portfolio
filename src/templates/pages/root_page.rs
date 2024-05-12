use axum::response::Html;
use axum::Extension;
use shtml::{html, Component, Render};

use crate::auth::cookies_and_jwt::LoggedInUser;
use crate::templates::attributes::Attrs::*;
use crate::templates::{components::button::Button, layout::RootLayout};

pub async fn root_page(payload: Option<Extension<LoggedInUser>>) -> Html<String> {
    tracing::debug!("Payload: {:?}", payload);
    let html_compnent = html! {
        <RootLayout>
            <Button props=[HxGet("/test"),
                      HxSwap("innerHTML transition:true"),
                      HxTarget("#swap-here")]>
                Call Test API
            </Button>
            <div id="swap-here"></div>
        </RootLayout>
    };
    Html(html_compnent.to_string())
}
