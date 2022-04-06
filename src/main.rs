extern crate dotenv;

use crate::routes::interaction::interaction_create::interaction_create;
use crate::routes::websocket::structures::client::{ClientBot, Interaction};
use crate::routes::websocket::websocket_server::websocket_message;
use dotenv::dotenv;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, Level};
use warp::ws::Ws;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

mod cryptography;
mod routes;
mod sign_mod;

type Clients = Arc<RwLock<HashMap<String, ClientBot>>>;
type Interactions = Arc<RwLock<HashMap<String, Interaction>>>;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    code_error: String,
    message: String,
    error: bool,
}

// HTTP
const INTERACTION_PING: u64 = 1;

// Interaction UI
const INTERACTION_COMMAND: u64 = 2;
const INTERACTION_BUTTON: u64 = 3;
const INTERACTION_AUTOCOMPLETE: u64 = 4;
const INTERACTION_MODAL_SUBMIT: u64 = 5;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load env
    let mut clients = Clients::default();
    let mut interactions = Interactions::default();
    tracing_subscriber::fmt().with_max_level(Level::WARN).init();

    let clients = warp::any().map(move || clients.clone());
    let interactions = warp::any().map(move || interactions.clone());
    let pub_key = warp::any().map(move || env::var("PUBLIC_KEY").unwrap().clone());

    let extern_api = warp::path::end().map(|| {
        warp::reply::json(
            &json!({ "status_code": 404, "message": "Not found!", "error": false }).as_object_mut(),
        )
    });

    let create_interaction = warp::path("interaction")
        .and(pub_key)
        .and(warp::header::header("X-Signature-Ed25519"))
        .and(warp::header::header("X-Signature-Timestamp"))
        .and(warp::body::content_length_limit(1024 * 900))
        .and(warp::body::json())
        .and(clients.clone())
        .and(interactions.clone())
        .and_then(interaction_create);
    let websocket_support = warp::path("gateway")
        .and(warp::ws())
        .and(warp::header::header("Identification"))
        .and(warp::header::header("Secret"))
        .and(warp::header::header("Public-Key"))
        .and(warp::header::header("Shard-In"))
        .and(warp::header::header("Shard-Total"))
        .and(clients.clone())
        .and(interactions.clone())
        .map(
            |ws: Ws,
             id: String,
             secret: String,
             pub_key_a: String,
             shard_in: String,
             shard_total: String,
             clients,
             interactions| {
                ws.on_upgrade(move |socket| {
                    websocket_message(
                        socket,
                        clients,
                        id,
                        secret,
                        shard_in.parse().unwrap(),
                        shard_total.parse().unwrap(),
                        interactions,
                        (
                            pub_key_a,
                            env::var("KEY_SECRET").unwrap(),
                            env::var("PUBLIC_KEY").unwrap(),
                            env::var("BOTS_DISCORD").unwrap(),
                        ),
                    )
                })
            },
        );

    let routes = warp::any()
        .and(extern_api.or(websocket_support).or(create_interaction))
        .recover(error_api)
        .with(warp::trace::request());

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

#[derive(Debug)]
struct Nope;

async fn error_api(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;
    let code_msg;

    if err.is_not_found() {
        message = "Could not find route";
        code = StatusCode::NOT_FOUND;
        code_msg = "NOT_FOUND";
    } else if let Some(_divide_by_zero) = err.find::<Nope>() {
        code = StatusCode::BAD_REQUEST;
        message = "It was not possible to make your request in the API.";
        code_msg = "BAD_REQUEST_API";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "There are errors in the metadata, please check it."
                } else {
                    "Invalid metadata."
                }
            }
            None => "Unknown error.",
        };

        code_msg = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_WRONG"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        code_msg = "METHOD_NOT_ALLOWED";
        message = "Method for this endpoint is invalid for this action.";
    } else {
        error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message =
            "Your request was rejected and therefore the API was unable to process your request.";
        code_msg = "UNHANDLED_REJECTION";
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&ErrorMessage {
            code: code.as_u16(),
            code_error: code_msg.to_string(),
            message: message.into(),
            error: true,
        }),
        code,
    ))
}
