# Tauri Axum
[![Crates.io](https://img.shields.io/crates/v/tauri-axum)](https://crates.io/crates/tauri-axum)
[![docs.rs](https://img.shields.io/docsrs/tauri-axum)](https://docs.rs/tauri-axum)



Run Axum web applications within Tauri, enabling server-side rendering patterns by running the Axum app in the Tauri backend.

## Quick Overview

Enables server-side rendering patterns by:

1. Intercepting HTTP requests in the webview
2. Forwarding them through Tauri's FFI bridge
3. Processing them with an Axum application running in the Tauri backend
4. Returning responses back to the webview to be handled by any frontend framework

Demo and example [source](example):


https://github.com/user-attachments/assets/02923cc7-281c-4271-9f52-02ecee1ac588


## Getting started

Create a vanilla Tauri project and add the library to your dependencies:

```toml
[dependencies]
tauri-axum = "0.1"
axum = "0.8"
tokio = { version = "1", features = ["full"] }
```

Set up your frontend to make HTTP requests through Tauri commands instead of external servers.

Create a Tauri command to process requests from the webview

```rust
struct TauriState {
    router: Arc<Mutex<Router>>,
}

#[tauri::command]
async fn local_app_request(
    state: State<'_, TauriState>,
    local_request: LocalRequest,
) -> Result<LocalResponse, ()> {
    let mut router = state.router.lock().await;

    let response = local_request.send_to_router(&mut router).await;

    Ok(response)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let tauri_stat = TauriState {
        router: Arc::new(Mutex::new(router)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(tauri_stat)
        .invoke_handler(tauri::generate_handler![local_app_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## SSE (Server-Sent Events) Support

The library now supports Server-Sent Events (SSE) for real-time streaming responses. You can create SSE responses using the `LocalResponse::sse()` method:

```rust
use tauri_axum::LocalResponse;

// Create an SSE response
let sse_data = "data: Hello World\n\ndata: Another message\n\n".as_bytes().to_vec();
let response = LocalResponse::sse(sse_data);
```

The library also provides helper functions for formatting SSE messages:

```rust
// Create SSE messages
let message = LocalResponse::sse_message("update", "Hello World");
let comment = LocalResponse::sse_comment("heartbeat");
let retry = LocalResponse::sse_retry(5000);
let event_id = LocalResponse::sse_event_id("123");
```

SSE responses are automatically detected by the `content-type: text/event-stream` header and will be properly flagged with the `is_sse()` method.
