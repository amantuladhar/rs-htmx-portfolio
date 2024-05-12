use axum::{extract::State, response::Html, Extension};
use shtml::{html, Component, Render};

use crate::{
    auth::cookies_and_jwt::LoggedInUser,
    repository::experience::Experience,
    templates::{attributes::Attrs, layout::RootLayout},
};

pub async fn update_portfolio_page(
    State(pool): State<sqlx::PgPool>,
    Extension(user): Extension<LoggedInUser>,
) -> Html<String> {
    let experiences = Experience::find_all(&pool, &user).await;
    let page = html! {
    <RootLayout props=[Attrs::LoggedInUser(Some(user))]>
        <ExperienceView experiences=experiences/>
    </RootLayout>
    };
    Html(page.to_string())
}

#[allow(non_snake_case)]
fn ExperienceView(experiences: Vec<Experience>) -> Component {
    let one = experiences.get(0).expect("at least one experience");
    html! {
        <div>
            <h1>{&one.title}</h1>
            <p>{&one.company}</p>
            <p>{&one.location}</p>
            <p>{&one.start_date.to_string()}</p>
            <p>{&one.end_date.map(|x| x.to_string()).unwrap_or("Current".to_string())}</p>
            <p>{&one.description}</p>
        </div>
    }
}
