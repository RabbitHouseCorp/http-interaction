extern crate warp;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use aes_gcm::aead::generic_array::typenum::And;
use crossbeam::sync::WaitGroup;
use serde_json::{json, Map, Value};
use tokio::sync::RwLock;
use warp::{Filter, Rejection};
use warp::path::Exact;
use warp::reply::{Json, WithStatus};
use warp::ws::Message;
use crate::{ClientBot, Clients, get_data, interaction_autocomplete, interaction_button, interaction_command, interaction_modal_submit, interaction_ping, Interactions, sign_mod};
use crate::routes::websocket::websocket_server::convert_to_binary;

// WithStatus<Json>
pub async fn interaction_create(sign: String, timestamp: String, json: HashMap<String, Value>, clients: Clients, interactions: Interactions) -> Result<impl warp::Reply, Infallible>
{
    let verify_sign = sign_mod::verify_authorization(String::from(""), sign, format!("{}{}", timestamp, json!(json)));
    match !verify_sign {
        true => {
            let type_interaction: &Value = json.get("type").unwrap();
            let type_int = type_interaction.as_u64().unwrap();
            if type_int == interaction_ping {
                return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 1 }).as_object_mut()), warp::http::StatusCode::OK));
            }
            if type_int == interaction_command {
                if json.get("application_id").is_none() {
                    return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE))
                }
                for (id, client) in clients.read().await.iter() {
                    if json.get("application_id").unwrap() == id {
                        if let Err(_disconnected) =  client.ws.tx.send(Message::binary(convert_to_binary(&json!(json)))) {

                               return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 4, "data": {
                                "tts": false,
								"content": "There was a problem with the interaction!",
                                "embeds": [
                                    {
                                        "color":       "#ff1212",
                                        "description": "The connection was lost by the bot. Contact bot developer."
                                    }
                                ],
                                   "allowed_mentions": []
                            } }).as_object_mut()), warp::http::StatusCode::OK));

                            break;
                        }
                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5 }).as_object_mut()), warp::http::StatusCode::OK));

                    }
                }

                eprintln!("Application offline");

                return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 4, "data": {
                                "tts": false,
								"content": "There was a problem with the interaction!",
                                "embeds": [
                                    {
                                        "color":       "#ff1212",
                                        "description": "I didn't get a response from the bot, try again or you can contact the developer through the support server."
                                    }
                                ],
                                "allowed_mentions": []
                    } }).as_object_mut()), warp::http::StatusCode::OK))

            }

            if type_int == interaction_button {
                if json.get("application_id").is_none() {
                    return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE))
                }

                for (id, client) in clients.read().await.iter() {
                    if json.get("application_id").unwrap() == id {
                        if let Err(_disconnected) = client.ws.tx.send(Message::binary(convert_to_binary(&json!(json)))) {
                            return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 4, "data": {
                                "tts": false,
								"content": "There was a problem with the interaction!",
                                "embeds": [
                                    {
                                        "color":       "#ff1212",
                                        "description": "The connection was lost by the bot. Contact bot developer."
                                    }
                                ],
                                   "allowed_mentions": []
                            } }).as_object_mut()), warp::http::StatusCode::OK));
                        }
                        tokio::time::sleep(Duration::from_millis(600)).await;

                        for (id, interaction) in interactions.read().await.iter() {
                            if id.to_string() == json.get("id").unwrap().to_string() {
                                let mut data_interaction = interaction.clone();
                                return Ok(warp::reply::with_status(warp::reply::json(&json!(data_interaction.data).as_object_mut()), warp::http::StatusCode::OK));
                            }
                        }

                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5 }).as_object_mut()), warp::http::StatusCode::OK));
                    }
                }

                if type_int == interaction_autocomplete {
                    if json.get("application_id").is_none() {
                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE))
                    }

                    for (id, client) in clients.read().await.iter() {
                        if json.get("application_id").unwrap() == id {
                            if let Err(_disconnected) = client.ws.tx.send(Message::binary(convert_to_binary(&json!(json)))) {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 4, "data": {
                                "tts": false,
								"content": "There was a problem with the interaction!",
                                "embeds": [
                                    {
                                        "color":       "#ff1212",
                                        "description": "The connection was lost by the bot. Contact bot developer."
                                    }
                                ],
                                   "allowed_mentions": []
                            } }).as_object_mut()), warp::http::StatusCode::OK));
                            }
                            tokio::time::sleep(Duration::from_millis(600)).await;

                            for (id, interaction) in interactions.read().await.iter() {
                                if id.to_string() == json.get("id").unwrap().to_string() {
                                    let mut data_interaction = interaction.clone();
                                    return Ok(warp::reply::with_status(warp::reply::json(&json!(data_interaction.data).as_object_mut()), warp::http::StatusCode::OK));
                                }
                            }

                            return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5 }).as_object_mut()), warp::http::StatusCode::OK));
                        }
                    }
                }
                match type_int {
                    interaction_autocomplete => {

                    }
                    interaction_modal_submit => {
                        if json.get("application_id").is_none() {
                            return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE))
                        }

                        for (id, client) in clients.read().await.iter() {
                            if json.get("application_id").unwrap() == id {
                                if let Err(_disconnected) = client.ws.tx.send(Message::binary(convert_to_binary(&json!(json)))) {
                                    return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 4, "data": {
                                "tts": false,
								"content": "There was a problem with the interaction!",
                                "embeds": [
                                    {
                                        "color":       "#ff1212",
                                        "description": "The connection was lost by the bot. Contact bot developer."
                                    }
                                ],
                                   "allowed_mentions": []
                            } }).as_object_mut()), warp::http::StatusCode::OK));
                                }
                                tokio::time::sleep(Duration::from_millis(600)).await;

                                for (id, interaction) in interactions.read().await.iter() {
                                    if id.to_string() == json.get("id").unwrap().to_string() {
                                        let mut data_interaction = interaction.clone();
                                        return Ok(warp::reply::with_status(warp::reply::json(&json!(data_interaction.data).as_object_mut()), warp::http::StatusCode::OK));
                                    }
                                }

                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 5,
                                     "components": [
                                        {
                                            "type": 1,
                                            "components": [
                                                {
                                                    "choices": []
                                                }
                                            ]
                                        }
                                    ]
                                }).as_object_mut()), warp::http::StatusCode::OK));
                            }
                        }
                    }
                    _ => {}
                }
            }


            Ok(
                warp::reply::with_status(warp::reply::json(&json!({ "status_code": 200, "message": "Interaction unknown or not recognized", "error": false, "code_error": "HTTP_INTERACTION_UNKNOWN" }).as_object_mut()), warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            )
        }
        false => {
            Ok(
                warp::reply::with_status(warp::reply::json(&json!({ "status_code": 401, "message": "Uh! It appears that this signature or metadata is incorrect. Check it out: https://discord.com/developers/docs/interactions/receiving-and-responding", "error": true, "code": "HTTP_UNAUTHORIZED" }).as_object_mut()), warp::http::StatusCode::UNAUTHORIZED)
            )
        }
    }
}