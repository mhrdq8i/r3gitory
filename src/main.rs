use axum::{
    Router,
    extract::Path,
    routing::{delete, get, post, put},
};

async fn root() -> &'static str {
    "Your are servig via Axum on port 3000"
}

async fn health() -> &'static str {
    "OK!"
}

async fn list_users() -> &'static str {
    "Listing all users"
}

async fn create_user() -> &'static str {
    "User created"
}

async fn get_user(Path(user_id): Path<String>) -> String {
    format!("Getting user: {}", user_id)
}

async fn update_user(Path(user_id): Path<String>) -> String {
    format!("Updated user: {}", user_id)
}

async fn delete_user(Path(user_id): Path<String>) -> String {
    format!("Deleted user: {}", user_id)
}

// Tokio Runtime
// #[tokio::main] — Macro that sets up async runtime
// Without this, you can't use await in main
#[tokio::main]
async fn main() {
    // Router
    // Router::new() — Creates empty router
    // .route("/", get(hello)) — When GET request hits /, call hello()
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/{user_id}",
            get(get_user).put(update_user).delete(delete_user),
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
