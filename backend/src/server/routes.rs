// server/routes.rs

// This file defines the routes for the Axum web server

// For your knowledge
// A route maps the HTTP request and a URL path to a specific handler function

use axum::{Router, routing::get};
use crate::server::handlers::root_handler;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
}