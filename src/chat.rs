#![allow(unused_assignments)]
use leptos::html::Input;
use leptos::*;
use leptos_use::storage::{use_local_storage, StringCodec};
use leptos_use::{use_websocket, UseWebsocketReturn};

#[component]
pub fn ChatPage() -> impl IntoView {
    let (username, _set_username, _reset) = use_local_storage::<String, StringCodec>("username");
    if username.get_untracked().is_empty() {
        // let navigate = leptos_router::use_navigate();
        // navigate("/", Default::default());
        #[cfg(target_family = "wasm")]
        if let Some(location) = document().location() {
            let _ = location.set_href("/");
        }
    }
    let change_username = move |_| {
        let navigate = leptos_router::use_navigate();
        navigate("/change-username", Default::default());
    };
    view! {
        <h1>"kakichat"</h1>
        <p>your usename is: {move || username.get()}</p>
        <ChatArea/>
        <br/>
        <button on:click=change_username >"Change username"</button>
    }
}

#[component]
fn ChatArea() -> impl IntoView {
    let (history, set_history) = create_signal(vec![]);

    let input_element: NodeRef<Input> = create_node_ref();

    let (username, _set_username, _reset) = use_local_storage::<String, StringCodec>("username");

    fn update_history(&history: &WriteSignal<Vec<String>>, message: String) {
        let chat = document()
            .get_element_by_id("chat")
            .expect("chatbox to exist");

        chat.set_scroll_top(chat.scroll_height());

        let _ = &history.update(|history: &mut Vec<_>| history.push(message));
    }

    let UseWebsocketReturn {
        ready_state,
        message,
        send,
        ..
    } = use_websocket(&format!("/ws/{}", username.get_untracked()));

    // let send_message = move |_| {
    //     let m = "Hello, world!";
    //     send(m);
    //     set_history.update(|history: &mut Vec<_>| history.push(format! {"[sent]: {:?}", m}));
    // };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let element = input_element().expect("<input> to exist");
        // here, we'll extract the value from the input
        let value = element.value();

        if value != "" {
            send(&value);
            set_history.update(|history: &mut Vec<_>| history.push(format! {"[sent]: {}", value}));
            //BAD: this is copied from line 39
            let chat = document()
                .get_element_by_id("chat")
                .expect("chatbox to exist");

            chat.set_scroll_top(chat.scroll_height());
        }
        element.set_value("");
    };
    let status = move || ready_state().to_string();

    create_effect(move |_| {
        if let Some(m) = message.get() {
            update_history(&set_history, m);
        };
    });

    view! {

        <p>"Status: " {status}</p>
        <div>
            <h2>"Chat:"</h2>
        </div>
        <div class="chatbox" id="chat">
            <For
                each=move || history.get().into_iter().enumerate()
                key=|(index, _)| *index
                let:item
            >
                <div>{item.1}</div>
            </For>
        </div>
        <form on:submit=on_submit class="chatbar">
            <input type="text"
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
    }
}
