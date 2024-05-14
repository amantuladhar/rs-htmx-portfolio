use axum::{extract::State, response::Html, Extension};
use axum_extra::extract::WithRejection;
use shtml::{html, Component, Render};

use crate::{
    auth::cookies_and_jwt::LoggedInUser,
    errors::api_error::ApiError,
    repository::{education::Education, experience::Experience},
    templates::{
        attributes::Attrs,
        components::button::{Button, ButtonVarient},
        layout::RootLayout,
        pages::update_portfolio::components::{EducationView, ExperienceView},
    },
};

pub async fn update_portfolio_page(
    State(pool): State<sqlx::PgPool>,
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
) -> Html<String> {
    let experiences = Experience::find_all(&pool, &user).await;
    let educations = Education::find_all(&pool, &user).await;
    let page = html! {
    <RootLayout props=[Attrs::LoggedInUser(&Some(user))]>
        <div class="flex flex-col gap-2">
            <div class="__experiences flex flex-col gap-2">
                <div class="flex sm:flex-row flex-col justify-between">
                    <h2 class="font-bold text-2xl mb-1">Experiences</h2>
                    <Button props=[
                        Attrs::Varient(ButtonVarient::Secondary),
                        Attrs::HxGet("/experiences/new"),
                        Attrs::HxSwap("innerHTML transition:true"),
                        Attrs::HxTarget("#presentation-body"),
                        Attrs::HxSelect(".__dialog"),
                        Attrs::HxPushUrl("true")
                    ]>Add Experience</Button>
                </div>
                <div class="grid lg:grid-cols-2 grid-cols-1 gap-2">
                    {
                        experiences.iter().map(|experience| {
                            html! {
                                <ExperienceView experience=&experience />
                            }
                        })
                        .collect::<Vec<_>>()
                    }
                </div>
            </div>
            <hr class="my-5" />
            <div class="__educations flex flex-col gap-2">
                <div class="flex sm:flex-row flex-col justify-between">
                    <h2 class="font-bold text-2xl mb-1">Educations</h2>
                    <Button props=[
                        Attrs::Varient(ButtonVarient::Secondary),
                        Attrs::HxGet("/educations/new"),
                        Attrs::HxSwap("innerHTML transition:true"),
                        Attrs::HxTarget("#presentation-body"),
                        Attrs::HxSelect(".__dialog"),
                        Attrs::HxPushUrl("true")
                    ]>Add Education</Button>
                </div>
                <div class="grid lg:grid-cols-2 grid-cols-1 gap-2">
                    {
                        educations.iter().map(|education| {
                            html! {
                                <EducationView education=&education />
                            }
                        })
                        .collect::<Vec<_>>()
                    }
                </div>
            </div>
        </div>
    </RootLayout>
    };
    Html(page.to_string())
}
