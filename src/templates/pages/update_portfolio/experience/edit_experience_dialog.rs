use axum::{
    extract::{Path, State},
    response::Html,
    Extension,
};
use axum_extra::extract::WithRejection;
use shtml::{html, Component, Render};

use crate::{
    auth::cookies_and_jwt::LoggedInUser,
    errors::api_error::ApiError,
    repository::experience::Experience,
    templates::{
        attributes::Attrs::*,
        components::{button::Button, dialog::Dialog, input::Input},
        layout::RootLayout,
    },
};

pub async fn edit_experience_dialog(
    State(pool): State<sqlx::PgPool>,
    Path(experience_id): Path<i32>,
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
) -> Html<String> {
    let experience = Experience::find_one(&pool, &user, experience_id).await;
    let user = Some(user);
    let html = html! {
        <AddEditExperienceForm title="Edit Experience" user=&user experience=&experience />
    };
    Html(html.to_string())
}

pub async fn add_experience_dialog(
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
) -> Html<String> {
    let experience = Experience::default();
    let user = Some(user);
    let html = html! {
        <AddEditExperienceForm title="Add Experience" user=&user experience=&experience />
    };
    Html(html.to_string())
}

#[allow(non_snake_case)]
fn AddEditExperienceForm(
    title: &str,
    user: &Option<LoggedInUser>,
    experience: &Experience,
) -> Component {
    let actions = html! {
        <Button props=[
            Type("submit"),
            Form("save_experience_form"),
            HxSwap("outerHTML transition:true"),
            HxTarget("#main-body"),
            HxSelect("#main-body"),
            HxPushUrl("true")
        ]>Save</Button>
        <Button props=[
            HxGet("/"),
            HxSwap("outerHTML transition:true"),
            HxTarget("#main-body"),
            HxSelect("#main-body"),
            // HxTrigger("click, keyup[key=='Escape'] from:body"),
            HxPushUrl("true")
        ]>Cancel</Button>
    };
    let html = html! {
        <RootLayout props=[LoggedInUser(user)]>
            <Dialog title=title actions=Some(actions) props=[Class("max-w-[600px] m-auto w-[80%]")]>
                <form id="save_experience_form"
                      hx-post="/experiences"
                      hx-target-error="#experience-form-error-section"
                      class="flex h-full flex-col justify-between gap-2 [&>*]:flex [&>*]:flex-col">
                    <input type="hidden" name="id" value=experience.id />
                    <div>
                        <label for="title">Job Title*</label>
                        <Input props=[Placeholder("Experience Title"),
                                    Name("title"),
                                    Required("true"),
                                    Value(&experience.title),
                                    Id("title")]  />
                    </div>
                    <div>
                        <label for="company">Company Name*</label>
                        <Input props=[Placeholder("Company Name"),
                                    Required("true"),
                                    Name("company"),
                                    Value(&experience.company),
                                    Id("company")]  />
                    </div>
                    <div>
                        <label for="location">Location*</label>
                        <Input props=[Placeholder("Location"),
                                    Name("location"),
                                    Required("true"),
                                    Value(&experience.location),
                                    Id("location")]  />
                    </div>
                    <div>
                        <label for="start_date">Start Date*</label>
                        <Input props=[Placeholder("Start Date"),
                                    Name("start_date"),
                                    Required("true"),
                                    Type("date"),
                                    Value(Experience::format_date_edit(&experience.start_date).as_str()),
                                    Id("start_date")]  />
                    </div>
                    <div>
                        <label for="end_date">End Date</label>
                        <Input props=[Placeholder("End Date"),
                                    Name("end_date"),
                                    Type("date"),
                                    Value(&experience.end_date.map(|x| Experience::format_date_edit(&x)).unwrap_or("".to_string())),
                                    Id("end_date")]  />
                    </div>
                    <div>
                        <label for="description">Description*</label>
                        <Input props=[Placeholder("Description"),
                                    Name("description"),
                                    Required("true"),
                                    Value(&experience.description),
                                    Id("description")]  />
                    </div>
                    <div id="experience-form-error-section">
                    </div>
                </form>
            </Dialog>
        </RootLayout>
    };
    html
}
