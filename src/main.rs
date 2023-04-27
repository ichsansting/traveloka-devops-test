use axum::{
    http::Method,
    routing::{get, post, patch, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

mod todo;

#[tokio::main]
async fn main() {
    todo::init::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", get(todo::get::get_all))
        .route("/todos/:id", get(todo::get::get))
        .route("/todos/create", post(todo::post::post))
        .route("/todos/:id/edit", patch(todo::patch::patch))
        .route("/todos/:id/delete", delete(todo::delete::delete))
        .route("/todos/create/last-id", get(todo::post::get_last_id))
        .layer(cors);

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
