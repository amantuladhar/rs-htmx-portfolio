#![allow(non_snake_case, unused)]

use shtml::{html, Component, Elements, Render};

use crate::templates::{attributes::Attrs, components::card::Card};

use super::button::Button;

#[derive(Default)]
pub struct DialogProps<'a> {
    class: &'a str,
    hx_ext: Option<&'a str>,
}

pub fn Dialog(
    title: &str,
    actions: Option<Elements>,
    props: impl IntoDialogProps,
    children: Elements,
) -> Component {
    let props = props.into_props();
    html! {
        <div class={r#"__dialog fixed top-0 left-0 h-[100dvh] w-[100dvw] bg-gray-600/70
             flex justify-center items-center z-[1000] "#}
             hx-ext={props.hx_ext}>
             <Card props=[Attrs::Class(format!("max-h-[90%] bg-white shadow-md flex flex-col gap-2 {}", props.class).as_str())]>
                <div class="__title font-bold text-2xl">{title}</div>
                <div class="__content overflow-scroll" > {children} </div>
                {actions.map(|action| { html! { <div class="__footer flex justify-end gap-2">{action}</div> } }).unwrap_or(html!{ "" })}
             </Card>
        </div>
    }
}

pub trait IntoDialogProps {
    fn into_props(&self) -> DialogProps;
}
impl<'a, const S: usize> IntoDialogProps for [Attrs<'a, ()>; S] {
    fn into_props(&self) -> DialogProps {
        let mut props = DialogProps::default();
        self.iter().for_each(|attr| match *attr {
            Attrs::Class(v) => props.class = v,
            Attrs::HxExt(v) => props.hx_ext = Some(v),
            _ => unimplemented!(),
        });
        props
    }
}
