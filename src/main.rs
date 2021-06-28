use std::str;
use warp::Filter;

use serde::{Deserialize, Serialize};

use warp::reply;

use http::status::StatusCode;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Rot13Query {
    //#[serde(rename = "boolshit")]
    text: String,
}

#[derive(Serialize)]
enum Rot13Reply {
    Success { rotated_text: String },
    Failed { error: String },
}

async fn main2() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    //let hello = warp::path!("rot13" / String).map(|name| format!("Hello, {}!", name));
    let hello = warp::path("rot13")
        .and(warp::body::json())
        //.and(warp::path::param::<String>())
        .map(
            |query: Rot13Query| handle_query(&query),
            //|query: Rot13Query| {
            //rot13(&(query.text)).unwrap_or(String::from("i no can handle special chars"))
            //}
        );

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

fn handle_query(query: &Rot13Query) -> reply::WithStatus<reply::Json> {
    let (json, status_code) = match rot13(&query.text) {
        Err(error) => (
            reply::json(&Rot13Reply::Failed {
                error: error.message,
            }),
            StatusCode::BAD_REQUEST,
        ),
        Ok(rotated_text) => (
            reply::json(&Rot13Reply::Success {
                rotated_text: rotated_text,
            }),
            StatusCode::OK,
        ),
    };

    warp::reply::with_status(json, status_code)
}

#[derive(Debug)]
struct Error {
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[tokio::main]
async fn main() {
    main2().await;
    println!("ntaienritae");
    println!("{}", rot13("Hallo").unwrap());
}

fn rot13(to_be_rotated: &str) -> Result<String, Error> {
    println!("{}", String::from("buuuuuuuuuuuuuuuuuuuuuuuuuuuh"));
    for char in to_be_rotated.chars() {
        println!("{}", char.to_ascii_lowercase());
        if !('a'..='z').contains(&char.to_ascii_lowercase()) {
            return Err(Error {
                message: String::from("Du musst zwischen a und z machen Brudi"),
            });
        }
    }

    Ok(to_be_rotated
        .chars()
        .into_iter()
        .map(|char| rot13_char(char))
        .collect())
}

fn rot13_char(char: char) -> char {
    let offset: u8;
    if ('a'..'z').contains(&char) {
        offset = 'a' as u8;
    } else if ('A'..'Z').contains(&char) {
        offset = 'A' as u8;
    } else {
        panic!("Ihr habt gemacht Bug lol.")
    }

    let char_as_byte = char as u8;

    ((((char_as_byte - offset) + 13) % 26) + offset) as char
}
