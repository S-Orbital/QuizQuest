use axum::extract::ws::WebSocket;
use std::collections::HashMap;

pub struct Game {
    game_id : String,
    websockets : Vec<WebSocket>,
    allowed_tokens : Vec<String>, // all player tokens that are currently in the game
    players : Vec<String>, // all current players in the game 
    player_scores : HashMap<String, i32>, // string for player name, i32 for point score
    host_usertoken : String,
    questions : HashMap<i8, (String, i8)> // first string in the question, 
                                               // second is the answer number

    
    // host user interaction should mostly be standard http request
    // based


}

