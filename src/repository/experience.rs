use crate::auth::cookies_and_jwt::LoggedInUser;

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
    pub async fn find_all(pool: &sqlx::PgPool, user: &LoggedInUser) -> Vec<Experience> {
        let experiences = sqlx::query_as!(
            Experience,
            "SELECT * FROM rs_portfolio_experience WHERE user_id = $1",
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
