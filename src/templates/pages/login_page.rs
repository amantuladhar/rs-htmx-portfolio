use axum::{
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;
use shtml::{html, Component, Render};

use crate::templates::{
    attributes::Attrs::*,
    components::{
        button::{Button, ButtonVarient},
        card::Card,
        input::Input,
        link::Link,
    },
    layout::RootLayout,
};

pub async fn login_page() -> Html<String> {
    let username_id = "login_username";
    let password_id = "login_password";
    let page = html! {
        <RootLayout>
            <Card props=[Class("max-w-[600px] m-auto w-[80%]"), HxExt("response-targets")]>
                <form hx-post="/login" class="[&>*]:flex [&>*]:flex-col flex flex-col gap-2"
                        hx-swap="innerHTML transition:true"
                        hx-target-error="#login-error-section">
                    <div>
                        <h2 class="font-bold text-4xl">Login</h2>
                    </div>
                    <div class="p-2"></div>
                    <div>
                        <label for=username_id>Username</label>
                        <Input props=[Placeholder("Username"), Name(username_id), Id(username_id)]  />
                    </div>
                    <div>
                        <label for=password_id>Password</label>
                        <Input props=[Name(password_id),
                                Id(password_id),
                                Placeholder("Password"),
                                Type("password")
                            ]/>
                    </div>
                    <div>
                        <Button props=[Varient(ButtonVarient::Secondary), Type("submit")]>Login</Button>
                    </div>
                    <div>
                        <span>"Don't have an account? "
                            <Link props=[Class("block underline"),
                                HxGet("/signup"),
                                HxTarget("#main-body"),
                                HxSelect("#main-body"),
                                HxPushUrl("true"),
                                HxSwap("innerHTML transition:true")]>
                                Sign Up
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
pub struct LoginReqBody {
    login_username: String,
    login_password: String,
}

pub async fn submit_login(Form(login): Form<LoginReqBody>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    if login.login_username == "fail" {
        let page = html! {
            <h2>FAIL Username = {login.login_username}</h2>
            <h2>FAIL Password = {login.login_password}</h2>
        };
        return (StatusCode::UNAUTHORIZED, headers, Html(page.to_string()));
    };
    headers.insert("hx-redirect", "/".parse().unwrap());
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
    headers.insert(
        "Set-Cookie",
        format!("APP=TOKEN-ID-{}; Secure; HttpOnly; SameSite=Strict", "aman")
            .parse()
            .unwrap(),
    );
    (StatusCode::OK, headers, Html("".to_string()))
}
