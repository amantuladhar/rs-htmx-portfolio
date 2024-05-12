use axum::{
    extract::rejection::ExtensionRejection,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};
use shtml::{html, Component, Render};
use thiserror::Error;
use tracing::debug;

use crate::{
    auth::cookies_and_jwt::create_cookie_with_exp_time,
    templates::{attributes::Attrs, components::button::Button, layout::RootLayout},
};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("🤬 Unauthorized")]
    Unauthorized(#[from] ExtensionRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        debug!("Got ApiError: {:?}", self);
        let mut header = HeaderMap::new();
        match self {
            ApiError::Unauthorized(_) => {
                header.insert("hx-redirect", "/login".parse().unwrap());
                header.insert(
                    "Set-Cookie",
                    create_cookie_with_exp_time("this-cookie-was-deleted".to_string(), 0)
                        .parse()
                        .unwrap(),
                );
                let html = html! {
                    <RootLayout props=[]>
                        <div class="grid place-items-center gap-5">
                            <div class="text-red-400"> {self.to_string()} </div>
                            <div> Redirecting to login page in 5 seconds </div>
                            <Button props=[Attrs::HxGet("/login"),
                                        Attrs::HxSwap("outerHTML transition:true"),
                                        Attrs::HxTarget("#main-body"),
                                        Attrs::HxSelect("#main-body"),
                                        Attrs::HxPushUrl("true"),
                                        Attrs::HxTrigger("load delay:5s, click")
                                        ]>Login</Button>
                        </div>
                    </RootLayout>
                }
                .to_string();
                return (StatusCode::UNAUTHORIZED, header, Html(html)).into_response();
            }
        };
    }
}
