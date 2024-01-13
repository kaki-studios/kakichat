use leptos::*;
use leptos_use::storage::{use_local_storage, StringCodec};

#[component]
pub fn ChatPage() -> impl IntoView {
    let (username, set_username, _reset) = use_local_storage::<String, StringCodec>("username");
    view! {
        <p>"chat page stub"</p>
        <p>your usename is: {move || username.get()}</p>
    }
}
