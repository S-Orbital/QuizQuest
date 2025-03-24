mod data;
mod db;

use std::collections::HashMap;

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade}, 
    response::{IntoResponse, Response}, 
    routing::{any, get, post}, 
    Json, 
    Router
};

use serde_json::{Value, json};
use once_cell::sync::Lazy;
use tokio::sync::{RwLock, Mutex};
use std::sync::Arc;


// declare websocket handler
// gameID, websocket vec 
static WEBSOCKET_MAP : Lazy<RwLock<HashMap<String, Vec<Arc<Mutex<WebSocket>>>>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});



#[tokio::main]
async fn main() {
    
    db::init();
    
    // 2 websocket endpoints 
    // one for host user
    // one for client

    let app = Router::new()
        .route("/", post(root));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(Json(payload): Json<serde_json::Value>) -> Json<Value>{

    Json(json!({"message": "this is a json return resposne"}))
}




