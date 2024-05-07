use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;
use shtml::{html, Component, Render};

use crate::{
    auth::{auth_middleware::AUTH_TOKEN_KEY, jwt_token::create_token, password::verify_password},
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
    utils::{anyhow_400, anyhow_err, bad_request},
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
                                HxGet("/login_body"),
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

pub async fn login_post_handler(
    State(pool): State<sqlx::PgPool>,
    Form(login): Form<LoginReqBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    validate_login_form_input(&login);
    let error_message = "Username or password is invalid".to_string();
    let db_value = sqlx::query!(
        "SELECT id, username, password FROM rs_portfolio_user WHERE username = $1",
        login.login_username
    )
    .fetch_optional(&pool)
    .await
    .map_err(bad_request)?
    .ok_or_else(|| anyhow::anyhow!(error_message.clone()))
    .map_err(|e| bad_request(&*e))?;

    if !verify_password(&db_value.password, &login.login_password).map_err(anyhow_400)? {
        return Err((StatusCode::BAD_REQUEST, error_message.clone()));
    }
    let token = create_token(db_value.id, &db_value.username).map_err(anyhow_err)?;

    let mut headers = HeaderMap::new();
    headers.insert("hx-redirect", "/".parse().unwrap());
    headers.insert(
        "Set-Cookie",
        format!(
            "{}={}; Secure; HttpOnly; SameSite=Strict",
            AUTH_TOKEN_KEY, token
        )
        .parse()
        .unwrap(),
    );
    Ok((StatusCode::OK, headers, Html("".to_string())))
}

fn validate_login_form_input(login: &LoginReqBody) -> Result<(), impl IntoResponse> {
    let mut error_html = vec![];
    if login.login_username.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Username cannot be blank
            </div>
        });
    }
    if login.login_password.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Password cannot be blank
            </div>
        });
    }
    if error_html.len() > 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            html! { { error_html } }.to_string(),
        ));
    }
    Ok(())
}
