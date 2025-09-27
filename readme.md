# Tauri Axum HTMX
[![Crates.io](https://img.shields.io/crates/v/tauri-axum-htmx)](https://crates.io/crates/tauri-axum-htmx)
[![docs.rs](https://img.shields.io/docsrs/tauri-axum-htmx)](https://docs.rs/tauri-axum-htmx)
[![npm version](https://img.shields.io/npm/v/tauri-axum-htmx.svg)](https://www.npmjs.com/package/tauri-axum-htmx) 
[![unpkg](https://img.shields.io/badge/unpkg-browse-blue)](https://unpkg.com/browse/tauri-axum-htmx/)



Build interactive UIs in Tauri applications using HTMX and Axum, enabling server-side rendering patterns by running the Axum app in the Tauri backend.

## Quick Overview

Enables server-side rendering patterns by:

1. Intercepting HTMX requests in the webview
2. Forwarding them through Tauri's FFI bridge
3. Processing them with an Axum application running in the Tauri backend
4. Returning HTML responses back to the webview be rendered by HTMX

Demo and example [source](example):


https://github.com/user-attachments/assets/02923cc7-281c-4271-9f52-02ecee1ac588


## Getting started

Create a vanilla Tauri project and initialize tauri-axum-htmx in src/index.html

```html
<!doctype html>
<html lang="en">
  <head>
    <script src="https://unpkg.com/htmx.org@2.0.4"></script>
    <script type="module">
      import { initialize } from "https://unpkg.com/tauri-axum-htmx";

      initialize("/"); // the initial path for the application to start on
    </script
  </head>
</html>
```

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

## JavaScript API

`initialize(initialPath: string, localAppRequestCommandOverride: string)`

- `initialPath`: The initial path for the application to start on
- `localAppRequestCommandOverride`: The name of the Tauri command to process requests from the webview

## SSE (Server-Sent Events) Support

The library now supports Server-Sent Events (SSE) for real-time streaming responses. You can create SSE responses using the `LocalResponse::sse()` method:

```rust
use tauri_axum_htmx::LocalResponse;

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
