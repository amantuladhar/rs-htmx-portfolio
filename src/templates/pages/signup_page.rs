use axum::{
    debug_handler,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;
use shtml::{html, Component, Render};
use sqlx::PgPool;

use crate::{
    templates::{
        attributes::Attrs::*,
        components::{
            button::{Button, ButtonVarient},
            card::Card,
            input::Input,
            link::Link,
        },
        layout::RootLayout,
    },
    utils::internal_error,
};

pub async fn signup_page() -> Html<String> {
    let username_id = "signup_username";
    let password_id = "signup_password";
    let page = html! {
        <RootLayout>
            <Card props=[Class("max-w-[600px] m-auto w-[80%]"), HxExt("response-targets")]>
                <form hx-post="/signup" class="[&>*]:flex [&>*]:flex-col flex flex-col gap-2"
                        hx-swap="innerHTML transition:true"
                        hx-target-error="#login-error-section">
                    <div>
                        <h2 class="font-bold text-4xl">SignUp</h2>
                    </div>
                    <div class="p-2"></div>
                    <div>
                        <label for=username_id>Username</label>
                        <Input props=[Placeholder("Username"), Name(username_id), Id(username_id), Required("true")]  />
                    </div>
                    <div>
                        <label for=password_id>Password</label>
                        <Input props=[Name(password_id),
                                Id(password_id),
                                Placeholder("Password"),
                                Type("password"),
                                Required("true")
                            ]/>
                    </div>
                    <div>
                        <Button props=[Varient(ButtonVarient::Secondary), Type("submit")]>Sign Up</Button>
                    </div>
                    <div>
                        <span>"Already have an account? "
                            <Link props=[Class("block underline"),
                                    HxSwap("innerHTML transition:true"),
                                    HxTarget("#main-body"),
                                    HxPushUrl("true"),
                                    HxSelect("#main-body"),
                                    HxGet("/login")]>
                                Login
                            </Link>
                        </span>
                    </div>
                    <div id="login-error-section">
                    </div>
                </form>
            </Card>
        </RootLayout>
    };
    Html(page.to_string())
}

#[derive(Deserialize)]
pub struct SignupRes {
    signup_username: String,
    signup_password: String,
}

#[debug_handler]
pub async fn signup_post_handler(
    State(pool): State<PgPool>,
    Form(signup): Form<SignupRes>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let mut error_html = vec![];
    if signup.signup_username.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Username cannot be blank
            </div>
        });
    }
    if signup.signup_password.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Password cannot be blank
            </div>
        });
    }
    if error_html.len() > 0 {
        return (
            StatusCode::BAD_REQUEST,
            headers,
            Html(html! { error_html }.to_string()),
        );
    }
    let id = sqlx::query!(
        "SELECT id FROM rs_portfolio_user WHERE username = $1 AND password = $2",
        signup.signup_username,
        signup.signup_password
    )
    .fetch_optional(&pool)
    .await
    .map_err(internal_error)
    .unwrap();

    let Some(record) = id else {
        return (
            StatusCode::BAD_REQUEST,
            headers,
            Html(html! {<div class="text-red-400">Invalid username or passsword</div>}.to_string()),
        );
    };
    headers.insert("hx-redirect", "/".parse().unwrap());
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
    headers.insert(
        "Set-Cookie",
        format!("APP=TOKEN-ID-{}; Secure; HttpOnly", record.id)
            .parse()
            .unwrap(),
    );
    (StatusCode::OK, headers, Html("".to_string()))
}
