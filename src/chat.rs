use leptos::*;

use super::UsernameContext;

#[component]
pub fn MainChat() -> impl IntoView {
    let username = use_context::<UsernameContext>().unwrap();
    if username.username.get_untracked() == "" {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    }
    view! {<p>{username.username}</p>}
}
