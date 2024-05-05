use axum::response::Html;
use shtml::{html, Component, Render};

use crate::templates::{
    attributes::Attrs::*,
    components::{card::Card, input::Input},
    layout::RootLayout,
};

pub async fn login_page() -> Html<String> {
    let html_compnent = html! {
        <RootLayout>
            <Card class="max-w-[600px] m-auto">
                <form method="POST" action="login" class="[&>*]:flex [&>*]:flex-col flex flex-col gap-2">
                    <div>
                        <label for="username">Username</label>
                        <Input props=[Class(""), Placeholder("Username 123")]  />
                    </div>
                    <div>
                        <label for="password">Password</label>
                        <input name="password" placeholder="password" type="password" />
                    </div>
                </form>
            </Card>
        </RootLayout>
    };
    Html(html_compnent.to_string())
}
