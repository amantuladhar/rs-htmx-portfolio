use shtml::{html, Component, Render};

use crate::templates::{components::Heading1, layout::RootLayout};

pub fn root_page() -> Component {
    html! {
        <RootLayout>
          <script src="https://unpkg.com/htmx.org@1.9.12"></script>
          <button hx-post="/clicked" hx-swap="outerHTML">
              Click Me
          </button>
          <Heading1 class="p-6 border border-black text-red-500 text-5xl">Test</Heading1>
        </RootLayout>
    }
}
