use axum::{extract::State, response::Html, Extension};
use axum_extra::extract::WithRejection;
use shtml::{html, Component, Render};

use crate::{
    auth::cookies_and_jwt::LoggedInUser,
    errors::api_error::ApiError,
    repository::experience::Experience,
    templates::{
        attributes::Attrs,
        components::{button::Button, card::Card},
        layout::RootLayout,
    },
};

pub async fn update_portfolio_page(
    State(pool): State<sqlx::PgPool>,
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
) -> Html<String> {
    let experiences = Experience::find_all(&pool, &user).await;
    let page = html! {
    <RootLayout props=[Attrs::LoggedInUser(Some(user))]>
        <div class="flex flex-col gap-2">
            <div class="__experiences">
                <h2 class="font-bold text-2xl mb-1">Experiences</h2>
                <div class="grid grid-cols-2 gap-2">
                    {
                        experiences.iter().map(|experience| {
                            html! {
                                <ExperienceView experience=&experience />
                            }
                        })
                        .collect::<Vec<_>>()
                    }
                    <Button props=[]>Add Experience</Button>
                    </div>
                    </div>
                    <div class="__experiences">
                    <h2 class="font-bold text-2xl mb-1">Projects</h2>
                    <div class="grid grid-cols-2 gap-2">
                    {
                        experiences.iter().map(|experience| {
                            html! {
                                <ExperienceView experience=&experience />
                            }
                        })
                        .collect::<Vec<_>>()
                    }
                    <Button props=[]>Add Projects</Button>
                </div>
            </div>
        </div>
    </RootLayout>
    };
    Html(page.to_string())
}

#[allow(non_snake_case)]
fn ExperienceView(experience: &Experience) -> Component {
    let start_date = experience.start_date.format("%B %Y").to_string();
    let end_date = experience
        .end_date
        .map(|x| x.format("%B %Y").to_string())
        .unwrap_or("Current".to_string());
    html! {
        <Card props=[]>
            <div class="flex h-full flex-col justify-between">
            <div class="__title font-bold">{format!("{} at {}, {}", &experience.title, &experience.company, &experience.location)}</div>
            <div class="__date text-gray-500 text-sm">{format!("{} - {}", start_date, end_date)}</div>
            <p>{&experience.description}</p>
            <div class="__footer flex justify-end">
                <Button props=[]>Edit</Button>
            </div>
            </div>
        </Card>
    }
}
