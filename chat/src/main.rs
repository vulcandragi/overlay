pub mod components;
pub mod services;

use yew::{Html, Renderer, component, html};

use crate::components::message::Messages;

#[component]
fn App() -> Html {
    html!(
        <Messages/>
    )
}

fn main() {
    Renderer::<App>::new().render();
}
