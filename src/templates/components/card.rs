#![allow(non_snake_case, unused)]

use shtml::{html, Component, Elements, Render};

use crate::templates::attributes::Attrs;

#[derive(Default)]
pub struct CardProps<'a> {
    class: &'a str,
    hx_ext: Option<&'a str>,
}

pub fn Card(props: impl IntoCardProps, children: Elements) -> Component {
    let props = props.into_props();
    html! {
        <div class={format!("border border-gray-200 p-4 {}", props.class)}
             hx-ext={props.hx_ext}>
            {children}
        </div>
    }
}

pub trait IntoCardProps {
    fn into_props(&self) -> CardProps;
}
impl<'a, const S: usize> IntoCardProps for [Attrs<'a, ()>; S] {
    fn into_props(&self) -> CardProps {
        let mut props = CardProps::default();
        self.iter().for_each(|attr| match *attr {
            Attrs::Class(v) => props.class = v,
            Attrs::HxExt(v) => props.hx_ext = Some(v),
            _ => unimplemented!(),
        });
        props
    }
}
