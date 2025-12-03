# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **tauri-axum-htmx**, a library that enables server-side rendering patterns in Tauri applications by using HTMX and Axum. The library intercepts HTMX requests in the webview, forwards them through Tauri's FFI bridge, processes them with an Axum application running in the Tauri backend, and returns HTML responses back to the webview.

## Key Commands

### Rust Development Commands
```bash
# Check code formatting
cargo fmt --all -- --check

# Build the library
cargo build

# Run tests
cargo test

# Run clippy for linting
cargo clippy --all-targets --all-features -- -D warnings

# Check with all features
cargo check --all-features
```

### JavaScript/Package Commands
```bash
# Copy xhr-fetch-proxy dependencies
npm run copy-pasta
```

### Example Applications
The repository contains two example applications:
- `example/axum-app/` - Standalone Axum web application
- `example/tauri-app/` - Tauri application using the library

## Architecture

### Core Components

**`src/lib.rs`** - Main library containing:
- `LocalRequest` struct: Represents HTTP requests that can be processed by Axum routers
- `LocalResponse` struct: Represents HTTP responses returned from Axum routers
- SSE (Server-Sent Events) support with helper methods for creating SSE responses
- Error handling with custom `Error` enum

### Key Patterns

1. **Request Processing Flow**:
   - HTMX requests are intercepted in the webview
   - Requests are serialized and sent through Tauri's FFI bridge
   - `LocalRequest::send_to_router()` converts the request to Axum format
   - Axum router processes the request
   - Response is converted back to `LocalResponse` format

2. **State Management**:
   - Tauri applications typically use a `TauriState` struct containing an `Arc<Mutex<Router>>`
   - The router is shared between Tauri commands and Axum handlers

3. **SSE Support**:
   - Library supports Server-Sent Events for real-time streaming
   - SSE responses are automatically detected by `content-type: text/event-stream`
   - Helper methods provided for formatting SSE messages

### Integration Pattern

Typical Tauri command structure:
```rust
#[tauri::command]
async fn local_app_request(
    state: State<'_, TauriState>,
    local_request: LocalRequest,
) -> Result<LocalResponse, ()> {
    let mut router = state.router.lock().await;
    let response = local_request.send_to_router(&mut router).await;
    Ok(response)
}
```

## Dependencies

Key Rust dependencies:
- `axum` - Web framework
- `serde` - Serialization/deserialization
- `tokio` - Async runtime
- `tower-service` - Service trait
- `thiserror` - Error handling

## Testing

The library includes comprehensive tests covering:
- Basic HTTP methods (GET, POST, PUT, DELETE, PATCH)
- Request/response handling with headers
- SSE response creation and detection
- Error handling for invalid requests
- Case-insensitive method parsing

Tests are organized into modules:
- `local_request_tests` - Tests for request processing
- `local_response_tests` - Tests for response handling and SSE
- `method_tests` - Tests for HTTP method validation