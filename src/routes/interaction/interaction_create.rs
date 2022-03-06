extern crate warp;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use aes_gcm::aead::generic_array::typenum::And;
use serde_json::{json, Map, Value};
use tokio::sync::RwLock;
use warp::{Filter, Rejection};
use warp::path::Exact;
use warp::reply::{Json, WithStatus};
use warp::ws::Message;
use crate::{ClientBot, Clients, get_data, HTTP_INTERACTION_CONFIRMATION_BOT, INTERACTION_COMMAND, sign_mod};
use crate::routes::websocket::websocket_server::convert_to_binary;

// WithStatus<Json>
pub async fn interaction_create(sign: String, timestamp: String, json: HashMap<String, Value>, clients: Clients) -> Result<impl warp::Reply, Infallible>
{
    let verify_sign = sign_mod::verify_authorization(String::from(""), sign, format!("{}{}", timestamp, json!(json)));
    match verify_sign {
        true => {
            let type_interaction: &Value = json.get("type").unwrap();
            let type_int = type_interaction.as_u64().unwrap();
            if type_int == HTTP_INTERACTION_CONFIRMATION_BOT {
                return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 1 }).as_object_mut()), warp::http::StatusCode::OK));
            }
            if type_int == INTERACTION_COMMAND {
                if json.get("application_id").is_none() {
                    return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE))
                }
                for (id, client) in clients.read().await.iter() {
                    if json.get("application_id").unwrap() == id {
                        if let Err(_disconnected) =  client.tx.send(Message::binary(convert_to_binary(&json!(json)))) {

                               return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 4, "data": {
                                "embeds": [
                                    {
                                        "description": "The connection was lost by the bot. Contact bot developer."
                                    }
                                ]
                            } }).as_object_mut()), warp::http::StatusCode::INTERNAL_SERVER_ERROR));

                            break;
                        }
                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5 }).as_object_mut()), warp::http::StatusCode::OK));

                    }
                }

                eprintln!("Application offline");

                return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 4, "data": {
                        "embeds": [
                            {
                                "description": "Application is offline! Try again later!"
                            }
                        ]
                    } }).as_object_mut()), warp::http::StatusCode::INTERNAL_SERVER_ERROR))

            }

            Ok(
                warp::reply::with_status(warp::reply::json(&json!({ "status_code": 200, "message": "Interaction unknown or not recognized", "error": false, "code_error": "HTTP_INTERACTION_UNKNOWN" }).as_object_mut()), warp::http::StatusCode::OK)
            )
        }
        false => {
            Ok(
                warp::reply::with_status(warp::reply::json(&json!({ "status_code": 401, "message": "Uh! It appears that this signature or metadata is incorrect. Check it out: https://discord.com/developers/docs/interactions/receiving-and-responding", "error": true, "code": "HTTP_UNAUTHORIZED" }).as_object_mut()), warp::http::StatusCode::UNAUTHORIZED)
            )
        }
    }
}