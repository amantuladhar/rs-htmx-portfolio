use axum::response::Html;
use axum::Extension;
use shtml::{html, Component, Render};

use crate::auth::cookies_and_jwt::LoggedInUser;
use crate::templates::attributes::Attrs::*;
use crate::templates::{components::button::Button, layout::RootLayout};

pub async fn root_page(user: Option<Extension<LoggedInUser>>) -> Html<String> {
    let html_compnent = html! {
        <RootLayout props=[LoggedInUser(&user.map(|u| u.0))]>
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
