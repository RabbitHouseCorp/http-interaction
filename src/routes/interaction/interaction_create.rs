extern crate dotenv;
extern crate warp;
use crate::routes::websocket::websocket_server::{convert_to_binary, search_shard};
use crate::{
    sign_mod, Clients, Interactions, INTERACTION_AUTOCOMPLETE, INTERACTION_BUTTON,
    INTERACTION_COMMAND, INTERACTION_MODAL_SUBMIT, INTERACTION_PING,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::convert::Infallible;
use std::time::Duration;
use warp::ws::Message;

pub async fn interaction_create(
    pub_key: String,
    sign: String,
    timestamp: String,
    json: HashMap<String, Value>,
    clients: Clients,
    interactions: Interactions,
) -> Result<impl warp::Reply, Infallible> {
    let keys_with_space = pub_key.split(" ");
    #[allow(unused_variables)]
    for key in keys_with_space {
        let verify_sign = sign_mod::verify_authorization(
            String::from(dotenv::var("PUBLIC_KEY").unwrap()),
            sign,
            format!("{}{}", timestamp, json!(json)),
        );
        match verify_sign {
            false => {
                let type_interaction: &Value = json.get("type").unwrap();
                let type_int = type_interaction.as_u64().unwrap();
                if type_int == INTERACTION_PING {
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&json!({ "type": 1 }).as_object_mut()),
                        warp::http::StatusCode::OK,
                    ));
                }
                if type_int == INTERACTION_COMMAND {
                    if json.get("application_id").is_none() {
                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE));
                    }
                    if json.get("guild_id").is_none() {
                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE));
                    }
                    #[allow(unreachable_code)]
                    for (id, client) in clients.read().await.iter() {
                        if json.get("application_id").unwrap() == id {
                            let shard_id = search_shard(
                                json.get("guild_id").unwrap().as_u64().unwrap() as usize,
                                client.ws.shard_total,
                            );
                            let find_shard = client.ws.shards.get(&shard_id.clone());
                            if find_shard.is_none() {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": "There was a problem with the interaction!",
                                        "embeds": [
                                            {
                                                "color": 0xff1212,
                                                "description": "It looks like the application is online but it's not yet available for your server. Wait 5 minutes to resume again."
                                            }
                                        ],
                                       "allowed_mentions": []
                                    }
                                }).as_object_mut()), warp::http::StatusCode::OK));
                            }
                            let get_shard = find_shard.unwrap();
                            if let Err(_disconnected) =
                                get_shard.send(Message::binary(convert_to_binary(&json!(json))))
                            {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": "There was a problem with the interaction!",
                                        "embeds": [
                                            {
                                                "color": 0xff1212,
                                                "description": "The connection was lost by the bot. Contact bot developer."
                                            }
                                        ],
                                       "allowed_mentions": []
                                    }
                                }).as_object_mut()), warp::http::StatusCode::OK));

                                break;
                            }
                            return Ok(warp::reply::with_status(
                                warp::reply::json(&json!({ "type": 5 }).as_object_mut()),
                                warp::http::StatusCode::OK,
                            ));
                        }
                    }

                    eprintln!("Application offline");

                    return Ok(warp::reply::with_status(warp::reply::json(&json!({
                        "type": 4,
                        "data": {
                            "tts": false,
                            "content": "There was a problem with the interaction!",
                            "embeds": [
                                {
                                    "color": 0xff1212,
                                    "description": "I didn't get a response from the bot, try again or you can contact the developer through the support server."
                                }
                            ],
                            "allowed_mentions": []
                        }
                    }).as_object_mut()), warp::http::StatusCode::OK));
                }

                if type_int == INTERACTION_BUTTON {
                    if json.get("application_id").is_none() {
                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE));
                    }

                    for (id, client) in clients.read().await.iter() {
                        if json.get("application_id").unwrap() == id {
                            let shard_id = search_shard(
                                json.get("guild_id").unwrap().as_u64().unwrap() as usize,
                                client.ws.shard_total,
                            );
                            let find_shard = client.ws.shards.get(&shard_id.clone());
                            if find_shard.is_none() {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": "There was a problem with the interaction!",
                                        "embeds": [
                                            {
                                                "color": 0xff1212,
                                                "description": "It looks like the application is online but it's not yet available for your server. Wait 5 minutes to resume again."
                                            }
                                        ],
                                       "allowed_mentions": []
                                    }
                                }).as_object_mut()), warp::http::StatusCode::OK));
                            }
                            let get_shard = find_shard.unwrap();
                            if let Err(_disconnected) =
                                get_shard.send(Message::binary(convert_to_binary(&json!(json))))
                            {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": "There was a problem with the interaction!",
                                        "embeds": [
                                            {
                                                "color": 0xff1212,
                                                "description": "The connection was lost by the bot. Contact bot developer."
                                            }
                                        ],
                                        "allowed_mentions": []
                                    }
                                }).as_object_mut()), warp::http::StatusCode::OK));
                            }
                            tokio::time::sleep(Duration::from_millis(400)).await;

                            for (id, interaction) in interactions.read().await.iter() {
                                if id.to_string() == json.get("id").unwrap().to_string() {
                                    let data_interaction = interaction.clone();
                                    return Ok(warp::reply::with_status(
                                        warp::reply::json(
                                            &json!(data_interaction.data).as_object_mut(),
                                        ),
                                        warp::http::StatusCode::OK,
                                    ));
                                }
                            }

                            return Ok(warp::reply::with_status(
                                warp::reply::json(&json!({ "type": 5 }).as_object_mut()),
                                warp::http::StatusCode::OK,
                            ));
                        }
                    }
                }

                if type_int == INTERACTION_AUTOCOMPLETE {
                    if json.get("application_id").is_none() {
                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE));
                    }

                    for (id, client) in clients.read().await.iter() {
                        if json.get("application_id").unwrap() == id {
                            let shard_id = search_shard(
                                json.get("guild_id").unwrap().as_u64().unwrap() as usize,
                                client.ws.shard_total,
                            );
                            let find_shard = client.ws.shards.get(&shard_id.clone());
                            if find_shard.is_none() {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": "There was a problem with the interaction!",
                                        "embeds": [
                                            {
                                                "color": 0xff1212,
                                                "description": "It looks like the application is online but it's not yet available for your server. Wait 5 minutes to resume again."
                                            }
                                        ],
                                       "allowed_mentions": []
                                    }
                                }).as_object_mut()), warp::http::StatusCode::OK));
                            }
                            let get_shard = find_shard.unwrap();
                            if let Err(_disconnected) =
                                get_shard.send(Message::binary(convert_to_binary(&json!(json))))
                            {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": "There was a problem with the interaction!",
                                        "embeds": [
                                            {
                                                "color": 0xff1212,
                                                "description": "The connection was lost by the bot. Contact bot developer."
                                            }
                                        ],
                                        "allowed_mentions": []
                                    }
                                }).as_object_mut()), warp::http::StatusCode::OK));
                            }

                            return Ok(warp::reply::with_status(
                                warp::reply::json(&json!({}).as_object_mut()),
                                warp::http::StatusCode::OK,
                            ));
                        }
                    }
                }

                if type_int == INTERACTION_MODAL_SUBMIT {
                    if json.get("application_id").is_none() {
                        return Ok(warp::reply::with_status(warp::reply::json(&json!({ "type": 5, "message_err": "API cannot accept this metadata because application was not included! Please resend again." }).as_object_mut()), warp::http::StatusCode::NOT_ACCEPTABLE));
                    }

                    for (id, client) in clients.read().await.iter() {
                        if json.get("application_id").unwrap() == id {
                            let shard_id = search_shard(
                                json.get("guild_id").unwrap().as_u64().unwrap() as usize,
                                client.ws.shard_total,
                            );
                            let find_shard = client.ws.shards.get(&shard_id.clone());
                            if find_shard.is_none() {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": "There was a problem with the interaction!",
                                        "embeds": [
                                            {
                                                "color": 0xff1212,
                                                "description": "It looks like the application is online but it's not yet available for your server. Wait 5 minutes to resume again."
                                            }
                                        ],
                                       "allowed_mentions": []
                                    }
                                }).as_object_mut()), warp::http::StatusCode::OK));
                            }
                            let get_shard = find_shard.unwrap();
                            if let Err(_disconnected) =
                                get_shard.send(Message::binary(convert_to_binary(&json!(json))))
                            {
                                return Ok(warp::reply::with_status(warp::reply::json(&json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": "There was a problem with the interaction!",
                                        "embeds": [
                                            {
                                                "color": 0xff1212,
                                                "description": "The connection was lost by the bot. Contact bot developer."
                                            }
                                        ],
                                         "allowed_mentions": []
                                    }
                                }).as_object_mut()), warp::http::StatusCode::OK));
                            }
                            tokio::time::sleep(Duration::from_millis(400)).await;

                            for (id, interaction) in interactions.read().await.iter() {
                                if id.to_string() == json.get("id").unwrap().to_string() {
                                    let data_interaction = interaction.clone();
                                    return Ok(warp::reply::with_status(
                                        warp::reply::json(
                                            &json!(data_interaction.data).as_object_mut(),
                                        ),
                                        warp::http::StatusCode::OK,
                                    ));
                                }
                            }

                            return Ok(warp::reply::with_status(
                                warp::reply::json(
                                    &json!({
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
                                    })
                                    .as_object_mut(),
                                ),
                                warp::http::StatusCode::OK,
                            ));
                        }
                    }
                }

                return Ok(
                    warp::reply::with_status(warp::reply::json(&json!({ "status_code": 200, "message": "Interaction unknown or not recognized", "error": false, "code_error": "HTTP_INTERACTION_UNKNOWN" }).as_object_mut()), warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                );
            }
            true => {
                return Ok(
                    warp::reply::with_status(warp::reply::json(&json!({ "status_code": 401, "message": "Uh! It appears that this signature or metadata is incorrect. Check it out: https://discord.com/developers/docs/interactions/receiving-and-responding", "error": true, "code": "HTTP_UNAUTHORIZED" }).as_object_mut()), warp::http::StatusCode::UNAUTHORIZED)
                );
            }
        }
    }
    Ok(warp::reply::with_status(warp::reply::json(&json!({ "status_code": 401, "message": "How strange, how are we going to open the door?", "error": true, "code": "HTTP_UNAUTHORIZED" }).as_object_mut()), warp::http::StatusCode::UNAUTHORIZED))
}
