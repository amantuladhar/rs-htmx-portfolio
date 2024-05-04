#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

use crate::templates::attributes::Attrs;

#[derive(Clone, Copy)]
pub enum ButtonVarient {
    Default,
    Danger,
}
#[derive(Default)]
pub struct ButtonProps<'a> {
    varient: ButtonVarient,
    class: &'a str,
    hx_post: Option<&'a str>,
    hx_swap: Option<&'a str>,
    hx_target: Option<&'a str>,
}

pub fn Button(props: impl IntoButtonProps, children: Elements) -> Component {
    use self::ButtonVarient::*;
    let props: ButtonProps = props.into_props();
    let varient = match props.varient {
        Default => "bg-black",
        Danger => "bg-red-400",
    };
    html! {
        <div class={format!("inline-block {varient}")}>
            <button hx-post={props.hx_post.unwrap_or("")}
                    hx-swap={props.hx_swap.unwrap_or("")}
                    hx-target={props.hx_target.unwrap_or("")}
                    class={format!(r#"border border-black bg-white px-3 py-2
                    transition-transform
                    hover:translate-x-[5px] hover:translate-y-[-5px]
                    {}"#, props.class)}>
                 {children}
             </button>
        </div>
    }
}

pub trait IntoButtonProps {
    fn into_props(&self) -> ButtonProps;
}

impl<'a> IntoButtonProps for Vec<Attrs<'a, ButtonVarient>> {
    fn into_props(&self) -> ButtonProps {
        let mut props = ButtonProps::default();
        self.iter().for_each(|attr| match *attr {
            Attrs::Varient(value) => props.varient = value,
            Attrs::Class(value) => props.class = value,
            Attrs::HxPost(value) => props.hx_post = Some(value),
            Attrs::HxSwap(value) => props.hx_swap = Some(value),
            Attrs::HxTarget(value) => props.hx_target = Some(value),
            #[allow(unreachable_patterns)]
            _ => {}
        });
        props
    }
}

impl Default for ButtonVarient {
    fn default() -> Self {
        ButtonVarient::Default
    }
}
