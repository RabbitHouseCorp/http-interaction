use std::{collections::{HashMap, hash_map::{DefaultHasher, RandomState}}, convert::Infallible, error::Error, fmt::format, hash::Hash};
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::{Value, json};
use warp::{Filter, Rejection, Reply, hyper::StatusCode};

use crate::structures::connection_state::ConnectionStateKraken;
mod structures;
mod gateway;
mod sign_mod;
mod constants;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
    error: bool
}
struct ResponseData {
    status_code: u64,
}

#[derive(Serialize, Deserialize)]
struct DiscordData {}

// HTTP
const HTTP_INTERACTION_CONFIRMATION_BOT: i32 = 1;

// Interaction UI
const INTERACTION_COMMAND: i32 = 2;
const INTERACTION_BUTTON: i32 = 3;


#[tokio::main]
async fn main() {
 //   let mut interactions = HashMap::new();
    let mut gateways = HashMap::new();
   // let mut bot_connection_state = HashMap::new();

    gateways.insert(String::from("2000"), ConnectionStateKraken{
        session_id: String::from(""),
        id: String::from(""),
        bot_id: String::from(""),
        master_shard: String::from(""),
        secret: String::from("everyone"),
        scope: vec![0],
        type_interaction: 0,
    });
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    
    let extern_api = warp::path::end().map(|| {
        warp::reply::json(
            &json!({ "status_code": 404, "message": "Not found!", "error": true }).as_object_mut(),
        )
    });
    
    let get_gateway = warp::path!("gateway" / u64).map(move |id: u64| {
        match gateways.contains_key(&format!("{}", id.to_string())) {
            false => warp::reply::json(
                &json!({ "status_code": 404, "message": "Not found!", "error": true }).as_object_mut(),
            ),
            true => warp::reply::json(
                &json!({ "status_code": 404, "data": gateways.get(&format!("{}", id.to_string())), "error": true })
                    .as_object_mut(),
            ),
        }
    });


    let create_interaction = warp::post()
        .and(warp::path("interaction"))
        .and(warp::header::header("X-Signature-Ed25519")) 
        .and(warp::header::header("X-Signature-Timestamp"))
        .and(warp::body::content_length_limit(1024 * 900))
        .and(warp::body::json())
        .map(|sign: String, timestamp: String, mut json: HashMap<String, Value>| {

            let a = sign_mod::verify_authorization(String::from(""), sign, format!("{}{:?}", timestamp, json));
            match a {
                false => {
                    let type_interaction: &Value = json.get("type").unwrap();

                    match type_interaction.as_u64().unwrap() {
                        HTTP_INTERACTION_CONFIRMATION_BOT => {
                            return warp::reply::with_status(warp::reply::json(&json!({ "type": 1 }).as_object_mut()), warp::http::StatusCode::OK);
                        }
                        _ => {}
                    }

                    warp::reply::with_status(warp::reply::json(&json!({ "status_code": 404, "message": "Not b!", "error": true }).as_object_mut()), warp::http::StatusCode::OK)
                }
                true => {
                    warp::reply::with_status(warp::reply::json(&json!({ "status_code": 404, "message": "Not a!", "error": true }).as_object_mut()), warp::http::StatusCode::UNAUTHORIZED)
                }
            }
        });
 
    
    let routes = warp::any()
    .and(
        extern_api
        .or(create_interaction)
        .or(get_gateway)
    )
    .recover(error_api);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug)]
struct Nope;

async fn error_api(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;
    
    if err.is_not_found() {
        message = "NOT_FOUND";
        code = StatusCode::NOT_FOUND;
    } else if let Some(DivideByZero) = err.find::<Nope>() {
        code = StatusCode::BAD_REQUEST;
        message = "BAD_REQUEST_API";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = match e.source() {
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
        message = "METHOD_NOT_ALLOWED";
    } else {

        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
        error: true,
    }), code))
}






