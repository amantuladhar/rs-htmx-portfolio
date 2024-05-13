use crate::{
    auth::cookies_and_jwt::LoggedInUser,
    templates::pages::update_portfolio::experience::edit_experience_post_handler::ExperienceInput,
};

pub struct Experience {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub company: String,
    pub location: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
}

impl Experience {
    pub async fn find_all(pool: &sqlx::PgPool, user: &LoggedInUser) -> Vec<Experience> {
        let experiences = sqlx::query_as!(
            Experience,
            "SELECT * FROM rs_portfolio_experience WHERE user_id = $1 ORDER BY start_date DESC",
            user.user_id
        )
        .fetch_all(pool)
        .await
        .expect("user experience to be returned");
        experiences
    }
    pub async fn find_one(
        pool: &sqlx::PgPool,
        user: &LoggedInUser,
        experience_id: i32,
    ) -> Experience {
        let experience = sqlx::query_as!(
            Experience,
            "SELECT * FROM rs_portfolio_experience WHERE id = $1 and user_id = $2",
            experience_id,
            user.user_id
        )
        .fetch_one(pool)
        .await
        .expect("experience to be returned");
        experience
    }

    pub async fn insert(
        pool: &sqlx::PgPool,
        user: &LoggedInUser,
        experience: &ExperienceInput,
    ) -> anyhow::Result<()> {
        if let Some(id) = experience.id.filter(|id| *id > 0) {
            return Err(anyhow::anyhow!("Experience has id({id}), cannot insert"));
        };
        sqlx::query!(
            r#"
            INSERT INTO rs_portfolio_experience (title, company, location, start_date, end_date, description, user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            experience.title,
            experience.company,
            experience.location,
            experience.start_date,
            experience.end_date,
            experience.description,
            user.user_id
        )
        .execute(pool)
        .await
        .expect("experience to be inserted");
        Ok(())
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        user: &LoggedInUser,
        experience: &ExperienceInput,
    ) -> anyhow::Result<()> {
        let Some(id) = experience.id.filter(|id| *id > 0) else {
            return Err(anyhow::anyhow!("Experience must have an id to update"));
        };
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
            experience.end_date,
            experience.description,
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
    pub fn format_date_edit(date: &chrono::NaiveDate) -> String {
        date.format("%Y-%m-%d").to_string()
    }
    pub fn format_date(date: chrono::NaiveDate) -> String {
        date.format("%B %Y").to_string()
    }
}

impl Default for Experience {
    fn default() -> Self {
        Self {
            id: 0,
            user_id: 0,
            title: "".to_string(),
            company: "".to_string(),
            location: "".to_string(),
            start_date: chrono::Utc::now().date_naive(),
            end_date: None,
            created_at: chrono::Utc::now(),
            description: "".to_string(),
        }
    }
}
