use crate::chat::*;
use leptos::ev::SubmitEvent;
use leptos::{html::Input, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::{use_local_storage, StringCodec};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to kakichat"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=RegisterPage/>
                    <Route path="/chat" view=ChatPage/>
                    <Route path="/change-username" view=ChangeUsernamePage/>
                    <Route path="/*any" view=NotFound/>

                </Routes>
            </main>
        </Router>
    }
}

///the page where new users land, they submit a username and start chatting
#[component]
fn RegisterPage() -> impl IntoView {
    let (username, set_username, _reset) = use_local_storage::<String, StringCodec>("username");
    if !username.get_untracked().is_empty() {
        let navigate = leptos_router::use_navigate();
        navigate("/chat", Default::default());
    }
    view! {
        <h1>"Welcome to kakichat!"</h1>
        <SetUsernameComponent/>
        <p>"Submit your username to start chatting!"</p>
    }
}

///the component where the user can change their username
#[component]
fn SetUsernameComponent() -> impl IntoView {
    let (username, set_username, _reset) = use_local_storage::<String, StringCodec>("username");
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
        if &value != &"" {
            set_username(value);
            let navigate = leptos_router::use_navigate();
            navigate("/chat", Default::default());
        }
    };
    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=username
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
    }
}

#[component]
fn ChangeUsernamePage() -> impl IntoView {
    view! {
        <h1>"Change Username"</h1>
        <SetUsernameComponent/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
