#![allow(non_snake_case)]

use shtml::{html, Component, Render};

use crate::templates::attributes::Attrs;

#[derive(Default)]
pub struct InputProps<'a> {
    class: &'a str,
    placeholder: Option<&'a str>,
    id: Option<&'a str>,
    name: Option<&'a str>,
    input_type: Option<&'a str>,
}

pub fn Input(props: impl IntoInputProps) -> Component {
    let props = props.into_props();
    html! {
        <input class={format!("border border-gray-400 px-2 py-2 {}", props.class)}
               placeholder={props.placeholder}
               name={props.name}
               type={props.input_type}
               id={props.id}
        />
    }
}

pub trait IntoInputProps {
    fn into_props(&self) -> InputProps;
}
impl<'a, const SIZE: usize> IntoInputProps for [Attrs<'a, ()>; SIZE] {
    fn into_props(&self) -> InputProps {
        let mut props = InputProps::default();
        self.iter().for_each(|attr| match *attr {
            Attrs::Class(v) => props.class = v,
            Attrs::Placeholder(v) => props.placeholder = Some(v),
            Attrs::Id(v) => props.id = Some(v),
            Attrs::Name(v) => props.name = Some(v),
            Attrs::Type(v) => props.input_type = Some(v),
            #[allow(unreachable_patterns)]
            _ => {}
        });
        props
    }
}
