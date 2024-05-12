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
    templates::{
        attributes::Attrs,
        components::{button::Button, dialog::Dialog},
        layout::RootLayout,
    },
};

pub async fn edit_experience(
    State(pool): State<sqlx::PgPool>,
    Path(experience_id): Path<i64>,
    WithRejection(Extension(user), _): WithRejection<Extension<LoggedInUser>, ApiError>,
) -> Html<String> {
    let html = html! {
        <RootLayout props=[Attrs::LoggedInUser(Some(user))]>
            <Dialog  props=[
                Attrs::Class("max-w-[600px] m-auto w-[80%]"),
            ] title="Dialog Test">
                <h1>{experience_id}</h1>
                <Button props=[
                    Attrs::HxGet("/update-portfolio"),
                    Attrs::HxSwap("outerHTML transition:true"),
                    Attrs::HxTarget("#main-body"),
                    Attrs::HxSelect("#main-body"),
                    Attrs::HxPushUrl("true")
                ]>Cancel</Button>
            </Dialog>
        </RootLayout>
    };
    Html(html.to_string())
}
