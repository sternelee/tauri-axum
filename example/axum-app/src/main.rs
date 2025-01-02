use axum_app::create_axum_app;

#[tokio::main]
async fn main() {
    let app = create_axum_app();

    println!("Server running on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
