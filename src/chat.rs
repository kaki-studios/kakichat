#![allow(unused_assignments)]
use leptos::html::Input;
use leptos::*;
use leptos_use::storage::{use_local_storage, StringCodec};
use leptos_use::{use_websocket, UseWebsocketReturn};

const SERVER_IP: &'static str = "141.145.204.255";

#[component]
pub fn ChatPage() -> impl IntoView {
    let (username, _set_username, _reset) = use_local_storage::<String, StringCodec>("username");
    if username.get_untracked().is_empty() {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
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
        let _ = &history.update(|history: &mut Vec<_>| history.push(message));
    }
    // ----------------------------
    // use_websocket
    // ----------------------------

    let UseWebsocketReturn {
        ready_state,
        message,
        send,
        ..
    } = use_websocket(&format!(
        "ws://{}:3000/ws/{}",
        SERVER_IP,
        username.get_untracked()
    ));

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

        <p>"status: " {status}</p>
        <div>
            <h3>"Chat:"</h3>
        </div>
        <div style="overflow:scroll; height:35em;">
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
