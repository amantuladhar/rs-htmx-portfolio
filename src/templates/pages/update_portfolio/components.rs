#![allow(non_snake_case)]

use shtml::{html, Component, Render};

use crate::{
    repository::{education::Education, experience::Experience},
    templates::{
        attributes::Attrs::*,
        components::{button::Button, card::Card},
    },
};

pub fn ExperienceView(experience: &Experience) -> Component {
    let start_date = experience.start_date.format("%B %Y").to_string();
    let end_date = experience
        .end_date
        .map(|x| x.format("%B %Y").to_string())
        .unwrap_or("Current".to_string());
    html! {
        <Card props=[]>
            <div class="flex h-full flex-col gap-2">
                <div class="__title font-bold">{format!("{} at {}, {}", &experience.title, &experience.company, &experience.location)}</div>
                <div class="__date text-gray-500 text-sm">{format!("{} - {}", start_date, end_date)}</div>
                <p class="break-words flex-1">{&experience.description}</p>
                <div class="__footer flex justify-end">
                    <Button props=[
                        HxGet(format!("/experiences/{}", experience.id).as_str()),
                        HxSwap("innerHTML transition:true"),
                        HxTarget("#presentation-body"),
                        HxSelect(".__dialog"),
                        HxPushUrl("true")
                    ]>Edit</Button>
                </div>
            </div>
        </Card>
    }
}

pub fn EducationView(education: &Education) -> Component {
    let start_date = education.formatted_start_date();
    let end_date = education.formatted_end_date();
    html! {
        <Card props=[]>
            <div class="flex h-full flex-col gap-2">
                <div class="__title font-bold">{format!("{} at {}, {}", &education.title, &education.school_name, &education.location)}</div>
                <div class="__date text-gray-500 text-sm">{format!("{} - {}", start_date, end_date)}</div>
                <p class="break-words flex-1">{&education.description}</p>
                <div class="__footer flex justify-end">
                    <Button props=[
                        HxGet(format!("/educations/{}", education.id).as_str()),
                        HxSwap("innerHTML transition:true"),
                        HxTarget("#presentation-body"),
                        HxSelect(".__dialog"),
                        HxPushUrl("true")
                    ]>Edit</Button>
                </div>
            </div>
        </Card>
    }
}
