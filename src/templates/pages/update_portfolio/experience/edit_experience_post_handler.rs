use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Extension, Form,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use shtml::{html, Component, Render};

use crate::{
    auth::cookies_and_jwt::LoggedInUser,
    errors::api_error::ApiError,
    repository::experience::Experience,
    utils::{anyhow_500, app_serde::optional_naive_date},
};

#[derive(Deserialize, Debug)]
pub struct ExperienceInput {
    pub id: Option<i32>,
    pub title: String,
    pub company: String,
    pub location: String,
    pub start_date: chrono::NaiveDate,
    #[serde(deserialize_with = "optional_naive_date")]
    pub end_date: Option<chrono::NaiveDate>,
    pub description: String,
}

pub async fn edit_experience_post_handler(
    State(pool): State<sqlx::PgPool>,
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
    Form(experience): Form<ExperienceInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    validate_experience_input(&experience)?;
    match experience.id.filter(|id| *id > 0) {
        Some(_) => Experience::update(&pool, &user, &experience)
            .await
            .map_err(anyhow_500)?,
        None => Experience::insert(&pool, &user, &experience)
            .await
            .map_err(anyhow_500)?,
    };
    let mut headers = HeaderMap::new();
    headers.insert("hx-redirect", "/update-portfolio".parse().unwrap());
    Ok((StatusCode::OK, headers, Html("".to_string())))
}

fn validate_experience_input(experience: &ExperienceInput) -> Result<(), (StatusCode, String)> {
    let mut error_html = vec![];
    if experience.title.trim() == "" {
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
