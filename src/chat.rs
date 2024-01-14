use leptos::*;
use leptos_use::storage::{use_local_storage, StringCodec};

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
    }
}
