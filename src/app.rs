#[path = "chat.rs"]
mod chat;

use crate::app::chat::*;
use crate::error_template::{AppError, ErrorTemplate};
use http::{HeaderValue, StatusCode};

use leptos::*;
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
    let sign_in = create_server_action::<SignIn>();

    view! {
        <ActionForm action=sign_in>
            <input type="text" name="username"/>
            <input type="submit" value="Sign In!"/>
        </ActionForm>
    }
}

//TODO: use localstorage instead!! link: https://leptos-use.rs/storage/use_local_storage.html
//TODO: chat will use websockets: check ~/git_clones/axum/examples/chat for an example. It needs
//the ws feature flag for axum!
//ALSO: for client-side websockets, see: https://leptos-use.rs/network/use_websocket.html
//TODO: style the webpage!! MAKE IT LOOK GOOD
//NOTE: try to keep code concise and dont jump to conclusions without researching first!
#[server(SignIn, "/api")]
pub async fn sign_in(username: String) -> Result<(), ServerFnError> {
    use leptos_axum::ResponseOptions;
    let response = expect_context::<ResponseOptions>();
    if username != "" {
        response.set_status(StatusCode::FOUND);
        response.insert_header(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&format!("username={} SameSite=None", username)).map_err(
                |_| ServerFnError::Deserialization("couldn\'t create cookie".to_string()),
            )?,
        );
        response.insert_header(
            axum::http::header::LOCATION,
            HeaderValue::from_str("/chat")
                .map_err(|_| ServerFnError::ServerError("could\'t redirect to /chat".into()))?,
        );
    } else {
        response.set_status(StatusCode::BAD_REQUEST);
    }
    Ok(())
}
