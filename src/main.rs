use axum::{
    Router,
    extract::Path,
    routing::{delete, get, post, put},
};

// Async Handler
// async fn — This function can pause and resume (needed for I/O)
// Returns &'static str — Axum automatically converts this to HTTP response
async fn hello() -> &'static str {
    "Hello R3gitory!"
}

async fn health() -> &'static str {
    "You are into the health function"
}

async fn version() -> &'static str {
    "You are into the version function"
}

async fn get_user ( Path(user_id): Path<String>) -> String {
    format!("Getting user with ID: {}", user_id)
}

async fn get_user_post ( Path((user_id, post_id)): Path<(String, String)>) -> String {
    format!("User {} - Post {}", user_id, post_id)
}

async fn create_user() -> &'static str {
    "Create a new user"
}

async fn update_user() -> &'static str {
    "Update a user"
}

async fn delete_user() -> &'static str {
    "Delete a user"
}

// Tokio Runtime
// #[tokio::main] — Macro that sets up async runtime
// Without this, you can't use await in main
#[tokio::main]
async fn main() {
    // Router
    // Router::new() — Creates empty router
    // .route("/", get(hello)) — When GET request hits /, call hello()
    let app = Router::new().route(
        "/users",
        get(get_user)
            .post(create_user)
            .put(update_user)
            .delete(delete_user),
    );
    // .route("/", get(hello))
    // .route("/health", get(health))
    // .route("/version", get(version));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
