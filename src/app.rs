#[path = "chat.rs"]
mod chat;

use crate::app::chat::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::html::Input;
use leptos::logging::log;
use leptos::{ev::SubmitEvent, *};
use leptos_meta::*;
use leptos_router::*;

#[derive(Debug, Clone)]
pub struct UsernameContext {
    pub username: ReadSignal<String>,
    pub set_username: WriteSignal<String>,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (username, set_username) = create_signal(String::from(""));

    provide_context(UsernameContext {
        username,
        set_username,
    });

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kakichat.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=SignInPage/>
                    <Route path="/chat" view=MainChat/>

                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn SignInPage() -> impl IntoView {
    let usernamecx = use_context::<UsernameContext>().unwrap();
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        (usernamecx.set_username)(value.clone());
        if value != "" {
            let navigate = leptos_router::use_navigate();
            navigate("/chat", Default::default());
        }
        //working
        //TODO: use cookies instead of context for persistence (simple, client-side, no validation)
    };

    view! {
        <h1> Hi! Welcome to kakichat! To start chatting, please enter a username:</h1>
        <form on:submit=on_submit>
            <input type="text"
                value=usernamecx.username
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>{usernamecx.username}</p>
    }
}
