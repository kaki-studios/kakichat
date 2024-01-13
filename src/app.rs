use crate::chat::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::html::Input;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::{use_local_storage, StringCodec};

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
    let (username, set_username, _reset) = use_local_storage::<String, StringCodec>("username");
    if !username.get().is_empty() {
        let navigate = leptos_router::use_navigate();
        navigate("/chat", Default::default());
    }
    let input_element: NodeRef<Input> = create_node_ref();
    use leptos::ev::SubmitEvent;
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
        set_username(value);
        let navigate = leptos_router::use_navigate();
        navigate("/chat", Default::default());
    };

    view! {
        // <input
        //     class="block"
        //     prop:value=move || username.get()
        //     on:input=move |e| set_username.update(|s| *s = event_target_value(&e))
        //     type="text"
        // />
        <form on:submit=on_submit>
            <input type="text"
                value=username
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Submit your username to start chatting!"</p>
    }
}

//TODO: use localstorage! link: https://leptos-use.rs/storage/use_local_storage.html
//TODO: chat will use websockets: check ~/git_clones/axum/examples/chat for an example. It needs
//the ws feature flag for axum!
//ALSO: for client-side websockets, see: https://leptos-use.rs/network/use_websocket.html
//TODO: style the webpage!! MAKE IT LOOK GOOD
//NOTE: try to keep code concise and dont jump to conclusions without researching first!
