#![allow(non_snake_case, unused)]

use shtml::{html, Component, Elements, Render};

use crate::templates::attributes::Attrs;

#[derive(Clone, Copy)]
pub enum ButtonVarient {
    Default,
    Secondary,
    Danger,
    Ghost,
}

#[derive(Default)]
pub struct ButtonProps<'a> {
    varient: ButtonVarient,
    class: &'a str,
    hx_get: Option<&'a str>,
    hx_post: Option<&'a str>,
    hx_swap: Option<&'a str>,
    hx_target: Option<&'a str>,
    hx_push_url: Option<&'a str>,
}

pub fn Button(props: impl IntoButtonProps, children: Elements) -> Component {
    use self::ButtonVarient::*;
    let props: ButtonProps = props.into_props();
    let varient_bg = match props.varient {
        Default | Ghost => "bg-black",
        Secondary => "bg-gray-500",
        Danger => "bg-red-400",
    };
    let varient_border = match props.varient {
        Default | Danger => "border border-black",
        Secondary => "border border-gray-400",
        Ghost => "border border-white hover:border-black",
    };
    let varient_btn_bg = match props.varient {
        Default | Danger | Ghost => "bg-white text-black",
        Secondary => "bg-black text-white",
    };
    html! {
        <div class={format!("inline-flex justify-stretch {varient_bg}")}>
            <button hx-post={props.hx_post}
                    hx-get={props.hx_get}
                    hx-swap={props.hx_swap}
                    hx-target={props.hx_target}
                    hx-push-url={props.hx_push_url}
                    class={format!(r#"{varient_border} flex-1 {varient_btn_bg} px-3 py-2
                        transition-transform hover:translate-x-[5px] hover:translate-y-[-5px]
                        {}"#, props.class)}>
                 {children}
             </button>
        </div>
    }
}

pub trait IntoButtonProps {
    fn into_props(&self) -> ButtonProps;
}

impl<'a, const SIZE: usize> IntoButtonProps for [Attrs<'a, ButtonVarient>; SIZE] {
    fn into_props(&self) -> ButtonProps {
        let mut props = ButtonProps::default();
        self.iter().for_each(|attr| match *attr {
            Attrs::Varient(value) => props.varient = value,
            Attrs::Class(value) => props.class = value,
            Attrs::HxPost(value) => props.hx_post = Some(value),
            Attrs::HxGet(value) => props.hx_get = Some(value),
            Attrs::HxSwap(value) => props.hx_swap = Some(value),
            Attrs::HxTarget(value) => props.hx_target = Some(value),
            Attrs::HxPushUrl(value) => props.hx_push_url = Some(value),
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
