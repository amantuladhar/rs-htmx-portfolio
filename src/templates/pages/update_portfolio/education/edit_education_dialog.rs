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
    repository::education::Education,
    templates::{
        attributes::Attrs::*,
        components::{button::Button, dialog::Dialog, input::Input},
        layout::RootLayout,
    },
};

pub async fn edit_education_dialog(
    State(pool): State<sqlx::PgPool>,
    Path(education_id): Path<i32>,
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
) -> Html<String> {
    let education = Education::find_one(&pool, &user, education_id).await;
    let user = Some(user);
    let html = html! {
        <AddEditEducationForm title="Edit Education" user=&user education=&education />
    };
    Html(html.to_string())
}

pub async fn add_education_dialog(
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
) -> Html<String> {
    let education = Education::default();
    let user = Some(user);
    let html = html! {
        <AddEditEducationForm title="Add Education" user=&user education=&education />
    };
    Html(html.to_string())
}

#[allow(non_snake_case)]
fn AddEditEducationForm(
    title: &str,
    user: &Option<LoggedInUser>,
    education: &Education,
) -> Component {
    let actions = html! {
        <Button props=[
            Type("submit"),
            Form("save_education_form"),
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
                <form id="save_education_form"
                      hx-post="/educations"
                      hx-target-error="#education-form-error-section"
                      class="flex h-full flex-col justify-between gap-2 [&>*]:flex [&>*]:flex-col">
                    <input type="hidden" name="id" value=education.id />
                    <div>
                        <label for="title">Job Title*</label>
                        <Input props=[Placeholder("Education Title"),
                                    Name("title"),
                                    Required("true"),
                                    Value(&education.title),
                                    Id("title")]  />
                    </div>
                    <div>
                        <label for="school_name">Company Name*</label>
                        <Input props=[Placeholder("School Name"),
                                    Required("true"),
                                    Name("school_name"),
                                    Value(&education.school_name),
                                    Id("school_name")]  />
                    </div>
                    <div>
                        <label for="location">Location*</label>
                        <Input props=[Placeholder("Location"),
                                    Name("location"),
                                    Required("true"),
                                    Value(&education.location),
                                    Id("location")]  />
                    </div>
                    <div>
                        <label for="start_date">Start Date*</label>
                        <Input props=[Placeholder("Start Date"),
                                    Name("start_date"),
                                    Required("true"),
                                    Type("date"),
                                    Value(Education::format_date_edit(&education.start_date).as_str()),
                                    Id("start_date")]  />
                    </div>
                    <div>
                        <label for="end_date">End Date</label>
                        <Input props=[Placeholder("End Date"),
                                    Name("end_date"),
                                    Type("date"),
                                    Value(&education.end_date.map(|x| Education::format_date_edit(&x)).unwrap_or("".to_string())),
                                    Id("end_date")]  />
                    </div>
                    <div>
                        <label for="description">Description*</label>
                        <Input props=[Placeholder("Description"),
                                    Name("description"),
                                    Required("true"),
                                    Value(&education.description),
                                    Id("description")]  />
                    </div>
                    <div id="education-form-error-section" class="text-red-500">
                    </div>
                </form>
            </Dialog>
        </RootLayout>
    };
    html
}
