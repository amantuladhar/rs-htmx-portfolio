use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Extension, Form,
};
use axum_extra::extract::WithRejection;
use shtml::{html, Component, Render};

use crate::{
    auth::cookies_and_jwt::LoggedInUser, errors::api_error::ApiError,
    repository::education::Education, utils::anyhow_500,
};

use super::EducationInput;

pub async fn edit_education_post_handler(
    State(pool): State<sqlx::PgPool>,
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
    Form(education): Form<EducationInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    validate_education_input(&education)?;
    match education.id.filter(|id| *id > 0) {
        Some(_) => Education::update(&pool, &user, &education)
            .await
            .map_err(anyhow_500)?,
        None => Education::insert(&pool, &user, &education)
            .await
            .map_err(anyhow_500)?,
    };
    let mut headers = HeaderMap::new();
    headers.insert("hx-redirect", "/".parse().unwrap());
    Ok((StatusCode::OK, headers, Html("".to_string())))
}

fn validate_education_input(education: &EducationInput) -> Result<(), (StatusCode, String)> {
    let mut error_html = vec![];
    if education.title.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Title cannot be blank
            </div>
        });
    }
    if education.school_name.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            School name cannot be blank
            </div>
        });
    }
    if education.location.trim() == "" {
        error_html.push(html! {
            <div class="text-red-400">
            Location cannot be blank
            </div>
        });
    }
    if education.description.trim() == "" {
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
