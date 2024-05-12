use std::str::FromStr;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Extension, Form,
};
use axum_extra::extract::WithRejection;
use chrono::NaiveDate;
use serde::Deserialize;
use shtml::{html, Component, Render};

use crate::{
    auth::cookies_and_jwt::LoggedInUser, errors::api_error::ApiError, utils::internal_error,
};

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
    State(pool): State<sqlx::PgPool>,
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
    Form(experience): Form<ExperienceInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    tracing::debug!("experience: {:?}", experience);
    validate_update_portfolio_post_handler(&experience)?;

    match experience.id.filter(|id| *id > 0) {
        Some(id) => {
            tracing::debug!("Upadting existing experience");
            sqlx::query!(
                r#"
                UPDATE rs_portfolio_experience
                SET title = $1, company = $2, location = $3, start_date = $4, end_date = $5, description = $6
                WHERE id = $7 and user_id = $8
                "#,
                experience.title,
                experience.company,
                experience.location,
                experience.start_date,
                experience
                    .end_date
                    .filter(|date| date.trim() != "")
                    .map(|date| NaiveDate::from_str(&date).unwrap()),
                experience.description,
                id,
                user.user_id
            )
            .execute(&pool)
            .await
            .map_err(internal_error)?;
        }
        None => {
            tracing::debug!("Inserting new experience");
            sqlx::query!(
                r#"
                INSERT INTO rs_portfolio_experience (title, company, location, start_date, end_date, description, user_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                experience.title,
                experience.company,
                experience.location,
                experience.start_date,
                experience
                    .end_date
                    .filter(|date| date.trim() != "")
                    .map(|date| NaiveDate::from_str(&date).unwrap()),
                experience.description,
                user.user_id
            )
            .execute(&pool)
            .await
            .map_err(internal_error)?;
        }
    };
    let mut headers = HeaderMap::new();
    headers.insert("hx-redirect", "/update-portfolio".parse().unwrap());
    Ok((StatusCode::OK, headers, Html("".to_string())))
}
fn validate_update_portfolio_post_handler(
    experience: &ExperienceInput,
) -> Result<(), (StatusCode, String)> {
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
    if let Some(end_date) = &experience.end_date {
        if end_date.trim() != "" {
            match NaiveDate::from_str(end_date) {
                Ok(_) => {}
                Err(_) => {
                    error_html.push(html! {
                        <div class="text-red-400">
                        End date is not a valid date
                        </div>
                    });
                }
            }
        }
    }
    if error_html.len() > 0 {
        let error_html = html! { { error_html } }.to_string();
        return Err((StatusCode::BAD_REQUEST, error_html));
    }
    Ok(())
}
