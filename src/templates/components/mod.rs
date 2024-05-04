#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

pub fn Heading1(class: &str, els: Elements) -> Component {
    html! { <h1 class=class>{els}</h1> }
}
