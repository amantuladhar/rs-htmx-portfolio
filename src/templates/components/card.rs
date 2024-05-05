#![allow(non_snake_case, unused)]

use shtml::{html, Component, Elements, Render};

pub fn Card(class: &str, children: Elements) -> Component {
    html! {
        <div class={format!("border border-gray-200 p-4 {class}")}>
            {children}
        </div>
    }
}
