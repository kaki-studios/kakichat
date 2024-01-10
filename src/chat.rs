use leptos::*;

use super::UsernameContext;

#[component]
pub fn MainChat() -> impl IntoView {
    let username = use_context::<UsernameContext>();
    view! {}
}
