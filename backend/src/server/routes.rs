// server/routes.rs

// This file defines the routes for the Axum web server

// For your knowledge
// A route maps the HTTP request and a URL path to a specific handler function

use axum::{Router, routing::{get, post}};
use crate::server::handlers::{root_handler, test_get_handler, test_post_handler};

// Function to create and return the router with all defined routes
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/test-get", get(test_get_handler))
        .route("/test-post", post(test_post_handler))
}