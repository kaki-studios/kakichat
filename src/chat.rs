use leptos::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::storage::{use_local_storage, StringCodec};
use leptos_use::{use_websocket, UseWebsocketReturn};

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
    //TODO: automatically connect to websocket server on page load
    view! {
        <h1>"kakichat"</h1>
        <p>your usename is: {move || username.get()}</p>
        <ChatArea/>
        <br/>
        <button on:click=change_username >"Change username"</button>
    }
}

#[component]
pub fn ChatArea() -> impl IntoView {
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        //TODO: send to websocket and clear input
    };
    view! {
        <div id="" style="overflow:scroll; height:35em;">
        //TODO: chat appears here!
        </div>

        <form on:submit=on_submit class="chatbar">
            <input type="text"
                // value=username
                // node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <h1>"TEST:"</h1>
        <TestChat/>
    }
}

#[component]
fn TestChat() -> impl IntoView {
    let (history, set_history) = create_signal(vec![]);

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
        message_bytes,
        send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket(&format!(
        //FIXME: Firefox can’t establish a connection to the server at ws://localhost:8000/ws/kaki/. leptos_start.js:763:12
        "ws://localhost:8000/ws/{}",
        username.get_untracked()
    ));

    let send_message = move |_| {
        let m = "Hello, world!";
        send(m);
        set_history.update(|history: &mut Vec<_>| history.push(format! {"[send]: {:?}", m}));
    };

    let send_byte_message = move |_| {
        let m = b"Hello, world!\r\n".to_vec();
        send_bytes(m.clone());
        set_history.update(|history: &mut Vec<_>| history.push(format! {"[send_bytes]: {:?}", m}));
    };

    let status = move || ready_state().to_string();

    let connected = move || ready_state.get() == ConnectionReadyState::Open;

    let open_connection = move |_| {
        open();
    };
    let close_connection = move |_| {
        close();
    };

    create_effect(move |_| {
        if let Some(m) = message.get() {
            update_history(&set_history, format! {"[message]: {:?}", m});
        };
    });

    create_effect(move |_| {
        if let Some(m) = message_bytes.get() {
            update_history(&set_history, format! {"[message_bytes]: {:?}", m});
        };
    });

    view! {

                    <h1 class="text-xl lg:text-4xl mb-2">"use_websocket"</h1>
                    <p>"status: " {status}</p>
                    <button on:click=send_message disabled=move || !connected()>
                        "Send"
                    </button>
                    <button on:click=send_byte_message disabled=move || !connected()>
                        "Send bytes"
                    </button>
                    <button on:click=open_connection disabled=connected>
                        "Open"
                    </button>
                    <button on:click=close_connection disabled=move || !connected()>
                        "Close"
                    </button>
                    <div class="flex items-center">
                        <h3 class="text-2xl mr-2">"History"</h3>
                        <button
                            on:click=move |_| set_history(vec![])
                            disabled=move || history.get().len() <= 0
                        >
                            "Clear"
                        </button>
                    </div>
                    <For
                        each=move || history.get().into_iter().enumerate()
                        key=|(index, _)| *index
                        let:item
                    >
                        <div>{item.1}</div>
                    </For>
    }
}
