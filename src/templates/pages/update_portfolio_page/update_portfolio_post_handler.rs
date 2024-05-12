use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Extension, Form,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use shtml::{html, Component, Render};

use crate::{auth::cookies_and_jwt::LoggedInUser, errors::api_error::ApiError};

#[derive(Deserialize, Debug)]
pub struct ExperienceInput {
    pub id: Option<i32>,
    pub title: String,
    pub company: String,
    pub location: String,
    pub start_date: chrono::NaiveDate,
    // TODO: Change to Option<chrono::NaiveDate>. Need to find how to handle empty value / string
    pub end_date: Option<String>,
    pub description: String,
}

pub async fn update_portfolio_post_handler(
    State(_pool): State<sqlx::PgPool>,
    WithRejection(Extension(_user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
    Form(experience): Form<ExperienceInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    tracing::debug!("experience: {:?}", experience);
    validate_update_portfolio_post_handler(&experience)?;
    // let error_message = "username or password is invalid".to_string();
    // let db_value = sqlx::query!(
    //     "SELECT id, username, password FROM rs_portfolio_user WHERE username = $1",
    //     login.login_username
    // )
    // .fetch_optional(&pool)
    // .await
    // .map_err(bad_request)?
    // .ok_or_else(|| anyhow::anyhow!(error_message.clone()))
    // .map_err(|e| bad_request(&*e))?;

    // if !verify_password(&db_value.password, &login.login_password).map_err(anyhow_400)? {
    //     return Err((StatusCode::BAD_REQUEST, error_message.clone()));
    // }

    // let token = create_token(db_value.id, &db_value.username).map_err(anyhow_500)?;
    let mut headers = HeaderMap::new();
    headers.insert("hx-redirect", "/".parse().unwrap());
    Ok((StatusCode::OK, headers, Html("".to_string())))
}
fn validate_update_portfolio_post_handler(
    experience: &ExperienceInput,
) -> Result<(), (StatusCode, String)> {
    let mut error_html = vec![];
    if experience.title.trim() == "asdf" {
        error_html.push(html! {
            <div class="text-red-400">
            Title cannot be blank
            </div>
        });
    }
    if experience.company.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Company cannot be blank
            </div>
        });
    }
    if experience.location.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Location cannot be blank
            </div>
        });
    }
    // if experience.start_date.trim() == "" {
    //     error_html.push(html! {
    //         <div class="text-red-400">
    //         Start date cannot be blank
    //         </div>
    //     });
    // }
    if experience.description.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Description cannot be blank
            </div>
        });
    }
    if error_html.len() > 0 {
        let error_html = html! { { error_html } }.to_string();
        return Err((StatusCode::BAD_REQUEST, error_html));
    }
    Ok(())
}
