#[macro_use] extern crate rocket;
mod db;

struct Question {
    question: String,
    images : Option<Vec<String>>, // contains image paths 
    a : String,
    b: String,
    c: String,
    d: String,
}

struct Game {
    host_user_token : String,
    game_id : String,
    player_token : Vec<String>, // string is userID/Token
}

struct player {
    player_token : String,
    // ws connection handle
}

#[get("/")]
fn index() -> &'static str {
    "server"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}




