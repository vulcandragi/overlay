use tonic::Request;
use wasm_bindgen_futures::spawn_local;
use yew::{AttrValue, Html, Properties, component, html, use_effect_with, use_state};

use crate::services::MessageService;

#[component]
pub fn Messages() -> Html {
    let messages = use_state(Vec::new);

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
                        current_messages.push(message);
                        current_messages
                    });
                }
            });

            || {}
        })
    }

    html!(
        <div>
        {
            for (*messages).iter().enumerate().map(|(i, message)| {
                let content_text = AttrValue::from(message.content.clone());
                html! {
                    <Message key={i} content={content_text} />
                }
            })
        }
        </div>
    )
}

#[derive(Properties, PartialEq)]
struct MessageProps {
    pub content: AttrValue,
}

#[component]
fn Message(props: &MessageProps) -> Html {
    html!(
        <div>{&props.content}</div>
    )
}
