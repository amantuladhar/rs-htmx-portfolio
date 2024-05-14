use serde::Deserialize;

use crate::utils::app_serde::optional_naive_date;

pub mod edit_education_dialog;
pub mod edit_education_post_handler;

#[derive(Deserialize, Debug)]
pub struct EducationInput {
    pub id: Option<i32>,
    pub title: String,
    pub school_name: String,
    pub location: String,
    pub start_date: chrono::NaiveDate,
    #[serde(deserialize_with = "optional_naive_date")]
    pub end_date: Option<chrono::NaiveDate>,
    pub description: String,
}
