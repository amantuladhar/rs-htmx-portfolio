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
}
