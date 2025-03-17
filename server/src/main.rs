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
    host_user : String,
    game_id : String,
    players : Vec<String>, // string is userID/Token
}

#[get("/")]
fn index() -> &'static str {
    "server"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}




