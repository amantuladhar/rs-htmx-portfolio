#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

use crate::templates::attributes::Attrs;

#[derive(Default)]
pub struct LinkProps<'a> {
    class: &'a str,
    hx_get: Option<&'a str>,
    hx_post: Option<&'a str>,
    hx_swap: Option<&'a str>,
    hx_target: Option<&'a str>,
    hx_push_url: Option<&'a str>,
    hx_select: Option<&'a str>,
}

pub fn Link(props: impl IntoLinkProps, children: Elements) -> Component {
    let props: LinkProps = props.into_props();
    html! {
        <div class="inline-block bg-black">
            <a  hx-post={props.hx_post}
                hx-get={props.hx_get}
                hx-swap={props.hx_swap}
                hx-target={props.hx_target}
                hx-push-url={props.hx_push_url}
                hx-select={props.hx_select}
                class={format!(r#"border border-white hover:border-black bg-white px-3 py-2
                        transition-transform hover:translate-x-[5px] hover:translate-y-[-5px]
                        {}"#, props.class)}>
                 {children}
             </a>
        </div>
    }
}

pub trait IntoLinkProps {
    fn into_props(&self) -> LinkProps;
}

impl<'a, const SIZE: usize> IntoLinkProps for [Attrs<'a, ()>; SIZE] {
    fn into_props(&self) -> LinkProps {
        let mut props = LinkProps::default();
        self.iter().for_each(|attr| match *attr {
            Attrs::Class(value) => props.class = value,
            Attrs::HxPost(value) => props.hx_post = Some(value),
            Attrs::HxGet(value) => props.hx_get = Some(value),
            Attrs::HxSwap(value) => props.hx_swap = Some(value),
            Attrs::HxTarget(value) => props.hx_target = Some(value),
            Attrs::HxPushUrl(value) => props.hx_push_url = Some(value),
            Attrs::HxSelect(value) => props.hx_select = Some(value),
            #[allow(unreachable_patterns)]
            _ => {}
        });
        props
    }
}
