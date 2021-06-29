use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::str;
use warp::reply;
use warp::Filter;

use crypto_leet_http_server_voll_cool::caesar::encrypt_caesar;
use crypto_leet_http_server_voll_cool::rot13::encrypt_rot13;

#[tokio::main]
async fn main() {
    let rot13 = warp::path("rot13")
        .and(warp::body::json())
        .map(|query: Rot13Query| handle_rot13_query(&query));

    let caesar = warp::path("caesar")
        .and(warp::body::json())
        .map(|query: CaesarQuery| handle_caesar_query(&query));

    warp::serve(rot13.or(caesar))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Rot13Query {
    text: String,
}

#[derive(Serialize)]
enum Rot13Reply {
    Success { rotated_text: String },
    Failed { error: String },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CaesarQuery {
    text: String,
    rotation: u8,
}

#[derive(Serialize)]
enum CaesarReply {
    Success { rotated_text: String },
    Failed { error: String },
}

fn handle_rot13_query(query: &Rot13Query) -> reply::WithStatus<reply::Json> {
    let (json, status_code) = match encrypt_rot13(&query.text) {
        Err(error) => (
            reply::json(&Rot13Reply::Failed {
                error: error.to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ),
        Ok(rotated_text) => (
            reply::json(&Rot13Reply::Success { rotated_text }),
            StatusCode::OK,
        ),
    };

    warp::reply::with_status(json, status_code)
}

fn handle_caesar_query(query: &CaesarQuery) -> reply::WithStatus<reply::Json> {
    let (json, status_code) = match encrypt_caesar(&query.text, query.rotation) {
        Err(error) => (
            reply::json(&CaesarReply::Failed {
                error: error.to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ),
        Ok(rotated_text) => (
            reply::json(&CaesarReply::Success { rotated_text }),
            StatusCode::OK,
        ),
    };

    warp::reply::with_status(json, status_code)
}
