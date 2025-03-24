// this is for our global websocket handling 

// Function to insert a WebSocket handle
async fn add_game(game_id: u64, ws: WebSocketStream<Upgraded>) {
    let mut games = GLOBAL_GAMES.write().await;
    games.insert(game_id, Arc::new(Mutex::new(ws)));
}

// Function to remove a game
async fn remove_game(game_id: u64) {
    let mut games = GLOBAL_GAMES.write().await;
    games.remove(&game_id);
}

// Function to get a WebSocket handle
async fn get_game_ws(game_id: u64) -> Option<Arc<Mutex<WebSocketStream<Upgraded>>>> {
    let games = GLOBAL_GAMES.read().await;
    games.get(&game_id).cloned()
}

// Example usage (sending a message)
async fn send_message(game_id: u64, msg: &str) {
    if let Some(ws) = get_game_ws(game_id).await {
        let mut ws = ws.lock().await;
        let _ = ws.send(tokio_tungstenite::tungstenite::protocol::Message::Text(msg.to_string())).await;
    }
}
