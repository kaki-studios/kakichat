use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::broadcast;

use leptos::*;
use leptos_use::{
    core::ConnectionReadyState,
    storage::{use_local_storage, StringCodec},
    use_websocket, UseWebsocketReturn,
};
#[component]
pub fn MainChat() -> impl IntoView {
    let (username, _set_username, _reset) = use_local_storage::<String, StringCodec>("username");

    if username.get().is_empty() {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    }
    let UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        send,
        open,
        close,
        ..
    } = use_websocket("wss://127.0.0.1/websocket");
    // let send_message = move |message: String| {
    //     send(&format!("{}: {}", username.get(), message));
    // };

    let send_message = move |_| {
        send(&format!("{}: {}", username.get(), "test!".to_string()));
    };
    let status = move || ready_state.get().to_string();

    let connected = move || ready_state.get() == ConnectionReadyState::Open;

    let open_connection = move |_| {
        open();
    };

    let close_connection = move |_| {
        close();
    };
    view! {
    <div>
            <p>"status: " {status}</p>

            <button on:click=send_message disabled=move || !connected()>"Send"</button>
            <button on:click=open_connection disabled=connected>"Open"</button>
            <button on:click=close_connection disabled=move || !connected()>"Close"</button>

            <p>"Receive message: " {move || format!("{:?}", message.get())}</p>
            <p>"Receive byte message: " {move || format!("{:?}", message_bytes.get())}</p>
        </div>
            <p>{move || username.get()}</p>
        }
}

// Our shared state
pub struct AppState {
    // Channel used to send messages to all connected clients.
    pub tx: broadcast::Sender<String>,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    use futures::{sink::SinkExt, stream::StreamExt};
    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            username = name;

            // If not empty we want to quit the loop else we want to quit function.
            break;
        }
    }

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);
}
