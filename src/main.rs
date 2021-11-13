use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use warp::{Filter, Rejection, Reply, hyper::StatusCode};
use crossbeam::sync::WaitGroup;
use crate::structures::connection_state::ConnectionStateKraken;
mod structures;
mod gateway;
mod sign_mod;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    code_error: String,
    message: String,
    error: bool
}
struct ResponseData {
    status_code: u64,
}

#[derive(Serialize, Deserialize)]
struct DiscordData {}

// HTTP
const HTTP_INTERACTION_CONFIRMATION_BOT: u64 = 1;

// Interaction UI
const INTERACTION_COMMAND: u64 = 2;
const INTERACTION_BUTTON: u64 = 3;

async fn get_data() {

}


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
        sync: WaitGroup::new().clone()
    });
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let extern_api = warp::path::end().map(|| {

        warp::reply::json(
            &json!({ "status_code": 404, "message": "Not found!", "error": false }).as_object_mut(),
        )
    });

    let get_gateway = warp::path!("gateway" / u64).map(move |id: u64| {
        match gateways.contains_key(&format!("{}", id.to_string())) {
            false => {
                return warp::reply::json(&json!({ "status_code": 404, "message": "Not found!", "error": true }).as_object_mut(),)
            }
            true => {
                gateways.get(&*id.to_string()).unwrap().sync.wait();
                return warp::reply::with_status(warp::reply::json(&json!({ "type": 1 }).as_object_mut()), warp::http::StatusCode::OK);
            }
        }
    });

    let test_gateway = warp::path!("gateway_test" / u64).map(move |id: u64| {
        match gateways.contains_key(&format!("{}", id.to_string())) {
            false => {
                return warp::reply::json(&json!({ "status_code": 404, "message": "Not found!", "error": true }).as_object_mut(),)
            }
            true => {
                gateways.get(&*id.to_string()).unwrap().sync.wait();

                Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 1 }).as_object_mut()), warp::http::StatusCode::OK);)
            }
        }
    });


    let create_interaction = warp::post()
        .and(warp::path("interaction"))
        .and(warp::header::header("X-Signature-Ed25519")) 
        .and(warp::header::header("X-Signature-Timestamp"))
        .and(warp::body::content_length_limit(1024 * 900))
        .and(warp::body::json())
        .map(|sign: String, timestamp: String, json: HashMap<String, Value>| {

            let verify_sign = sign_mod::verify_authorization(String::from(""), sign, format!("{}{}", timestamp, json!(json)));
            match verify_sign {
                true => {
                    let type_interaction: &Value = json.get("type").unwrap();

                    match type_interaction.as_u64().unwrap() {
                        HTTP_INTERACTION_CONFIRMATION_BOT => {
                            return warp::reply::with_status(warp::reply::json(&json!({ "type": 1 }).as_object_mut()), warp::http::StatusCode::OK);
                        }
                        INTERACTION_COMMAND => {
                            return warp::reply::with_status(warp::reply::json(&json!({ "type": 5 }).as_object_mut()), warp::http::StatusCode::OK);
                        }
                        INTERACTION_BUTTON => {
                            async {
                                let data = get_data().await;
                                return warp::reply::with_status(warp::reply::json(&json!(data).as_object_mut()), warp::http::StatusCode::OK);
                            };
                        }
                        _ => {}
                    }

                    warp::reply::with_status(warp::reply::json(&json!({ "status_code": 200, "message": "Interaction unknown or not recognized", "error": false, "code_error": "HTTP_INTERACTION_UKNOWN" }).as_object_mut()), warp::http::StatusCode::OK)
                }
                false => {
                    warp::reply::with_status(warp::reply::json(&json!({ "status_code": 401, "message": "Uh! It appears that this signature or metadata is incorrect. Check it out: https://discord.com/developers/docs/interactions/receiving-and-responding", "error": true, "code": "HTTP_UNAUTHORIZED" }).as_object_mut()), warp::http::StatusCode::UNAUTHORIZED)
                }
            }
        });

    let routes = warp::any()
    .and(
        extern_api
        .or(create_interaction)
        .or(get_gateway)
            .or(test_gateway)
    )
    .recover(error_api);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
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
    } else if let Some(_DivideByZero) = err.find::<Nope>() {
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

        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Your request was rejected and therefore the API was unable to process your request.";
        code_msg = "UNHANDLED_REJECTION";
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        code_error: code_msg.to_string(),
        message: message.into(),
        error: true,
    }), code))
}






