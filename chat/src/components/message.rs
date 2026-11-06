use std::collections::VecDeque;

use stylist::yew::use_style;
use tonic::Request;
use wasm_bindgen_futures::spawn_local;
use yew::{AttrValue, Html, Properties, component, html, use_effect_with, use_state};

use crate::services::MessageService;

#[component]
pub fn Messages() -> Html {
    let messages = use_state(VecDeque::new);

    {
        let messages = messages.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let mut message_service = MessageService::new();

                let Ok(response) = message_service.client.listen(Request::new(())).await else {
                    return;
                };

                let mut stream = response.into_inner();

                while let Ok(Some(message)) = stream.message().await {
                    messages.set({
                        let mut current_messages = (*messages).clone();
                        if current_messages.len() == 5 {
                            current_messages.pop_front();
                        }

                        current_messages.push_back(message);
                        current_messages
                    });
                }
            });

            || {}
        })
    }

    let css = use_style!(
        r#"
            position: absolute;
            bottom: 0;
            padding: 10px;
        "#
    );

    html!(
        <div class={css}>
        {
            for (*messages).iter().map(|message| {
                let props = MessageProps {
                    content: AttrValue::from(message.content.clone()),
                    name: AttrValue::from(message.name.clone()),
                    name_color: AttrValue::from(message.name_color.clone())
                };

                html! {
                    <Message key={message.message_id.clone()} ..props />
                }
            })
        }
        </div>
    )
}

#[derive(Properties, PartialEq)]
struct MessageProps {
    pub content: AttrValue,
    pub name: AttrValue,
    pub name_color: AttrValue,
}

#[component]
fn Message(props: &MessageProps) -> Html {
    let container_css = use_style!(
        r#"
            position: relative;
        "#
    );

    let name_css = use_style!(
        r#"
            background-color: #000000;
            width: fit-content;
            color: var(--name-color, #ffffff);
            padding: 8px;
            border-radius: 5px;
        "#
    );
    let name_css_vars = format!("--name-color: {}", props.name_color);

    let content_css = use_style!(
        r#"
            background-color: #eeadad;
            width: fit-content;
            padding: 8px;
            border-radius: 5px;
            position: relative;
            max-width: 300px;
            top: -5px;
            left: 20px;
            z-index: -1;
        "#
    );

    html!(
        <div class={container_css}>
            <div class={name_css} style={name_css_vars}>{&props.name}</div>
            <div class={content_css}>{&props.content}</div>
        </div>
    )
}
