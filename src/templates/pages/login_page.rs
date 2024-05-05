use axum::response::Html;
use shtml::{html, Component, Render};

use crate::templates::{
    attributes::Attrs::*,
    components::{
        button::{Button, ButtonVarient},
        card::Card,
        input::Input,
    },
    layout::RootLayout,
};

pub async fn login_page() -> Html<String> {
    let username_id = "login-username";
    let password_id = "login-password";
    let page = html! {
        <RootLayout>
            <Card class="max-w-[600px] m-auto w-[80%]">
                <form method="POST" action="login" class="[&>*]:flex [&>*]:flex-col flex flex-col gap-2">
                    <div>
                        <h2 class="font-bold text-4xl">Login</h2>
                    </div>
                    <div class="p-2"></div>
                    <div>
                        <label for=username_id>Username</label>
                        <Input props=[Placeholder("Username"), Name(username_id), Id(username_id)]  />
                    </div>
                    <div>
                        <label for=password_id>Password</label>
                        <Input props=[Name(password_id),
                                Id(password_id),
                                Placeholder("Password"),
                                Type("password")
                            ]/>
                    </div>
                    <div>
                        <Button props=[Varient(ButtonVarient::Secondary)]>Login</Button>
                    </div>
                </form>
            </Card>
        </RootLayout>
    };
    Html(page.to_string())
}
