use crate::{
    auth::cookies_and_jwt::LoggedInUser,
    templates::pages::update_portfolio::education::EducationInput,
};

pub struct Education {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub school_name: String,
    pub location: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
}

impl Education {
    pub async fn find_all(pool: &sqlx::PgPool, user: &LoggedInUser) -> Vec<Self> {
        let educations = sqlx::query_as!(
            Self,
            "SELECT * FROM rs_portfolio_education WHERE user_id = $1 ORDER BY start_date DESC",
            user.user_id
        )
        .fetch_all(pool)
        .await
        .expect("user education to be returned");
        educations
    }
    pub async fn find_one(pool: &sqlx::PgPool, user: &LoggedInUser, education_id: i32) -> Self {
        let education = sqlx::query_as!(
            Self,
            "SELECT * FROM rs_portfolio_education WHERE id = $1 and user_id = $2",
            education_id,
            user.user_id
        )
        .fetch_one(pool)
        .await
        .expect("education to be returned");
        education
    }

    pub async fn insert(
        pool: &sqlx::PgPool,
        user: &LoggedInUser,
        education: &EducationInput,
    ) -> anyhow::Result<()> {
        if let Some(id) = education.id.filter(|id| *id > 0) {
            return Err(anyhow::anyhow!("Self has id({id}), cannot insert"));
        };
        sqlx::query!(
            r#"
            INSERT INTO rs_portfolio_education (title, school_name, location, start_date, end_date, description, user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            education.title,
            education.school_name,
            education.location,
            education.start_date,
            education.end_date,
            education.description,
            user.user_id
        )
        .execute(pool)
        .await
        .expect("education to be inserted");
        Ok(())
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        user: &LoggedInUser,
        education: &EducationInput,
    ) -> anyhow::Result<()> {
        let Some(id) = education.id.filter(|id| *id > 0) else {
            return Err(anyhow::anyhow!("Self must have an id to update"));
        };
        sqlx::query!(
            r#"
            UPDATE rs_portfolio_education
            SET title = $1, school_name = $2, location = $3, start_date = $4, end_date = $5, description = $6
            WHERE id = $7 and user_id = $8
            "#,
            education.title,
            education.school_name,
            education.location,
            education.start_date,
            education.end_date,
            education.description,
            id,
            user.user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn formatted_start_date(&self) -> String {
        Self::format_date(self.start_date)
    }
    #[allow(dead_code)]
    pub fn formatted_end_date(&self) -> String {
        self.end_date
            .map(Self::format_date)
            .unwrap_or("Current".to_string())
    }
    #[allow(dead_code)]
    pub fn format_date_edit(date: &chrono::NaiveDate) -> String {
        date.format("%Y-%m-%d").to_string()
    }
    pub fn format_date(date: chrono::NaiveDate) -> String {
        date.format("%B %Y").to_string()
    }
}

impl Default for Education {
    fn default() -> Self {
        Self {
            id: 0,
            user_id: 0,
            title: "".to_string(),
            school_name: "".to_string(),
            location: "".to_string(),
            start_date: chrono::Utc::now().date_naive(),
            end_date: None,
            created_at: chrono::Utc::now(),
            description: "".to_string(),
        }
    }
}
