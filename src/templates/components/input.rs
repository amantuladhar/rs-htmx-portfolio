#![allow(non_snake_case, unused)]

use shtml::{html, Component, Elements, Render};

use crate::templates::attributes::Attrs;

#[derive(Default)]
pub struct InputProps<'a> {
    varient: InputVarient,
    class: &'a str,
    placeholder: Option<&'a str>,
}

pub fn Input(props: impl IntoInputProps) -> Component {
    let props = props.into_props();
    html! {
        <input class={format!("border border-black {}", props.class)} placeholder={props.placeholder} />
    }
}

pub enum InputVarient {
    Default,
}
impl Default for InputVarient {
    fn default() -> Self {
        InputVarient::Default
    }
}

pub trait IntoInputProps {
    fn into_props(&self) -> InputProps;
}
impl<'a, const SIZE: usize> IntoInputProps for [Attrs<'a, InputVarient>; SIZE] {
    fn into_props(&self) -> InputProps {
        let mut props = InputProps::default();
        self.iter().for_each(|attr| match *attr {
            Attrs::Class(value) => props.class = value,
            Attrs::Placeholder(value) => props.placeholder = Some(value),
            #[allow(unreachable_patterns)]
            _ => {}
        });
        props
    }
}
