extern crate warp;
use std::collections::HashMap;
use aes_gcm::aead::generic_array::typenum::And;
use serde_json::{json, Map, Value};
use warp::{Filter, Rejection};
use warp::path::Exact;
use warp::reply::{Json, WithStatus};
use crate::{get_data, HTTP_INTERACTION_CONFIRMATION_BOT, INTERACTION_COMMAND, sign_mod};

pub fn interaction_create(sign: String, timestamp: String, json: HashMap<String, Value>) -> WithStatus<Json> {
    let verify_sign = sign_mod::verify_authorization(String::from(""), sign, format!("{}{}", timestamp, json!(json)));
    match verify_sign {
        true => {
            let type_interaction: &Value = json.get("type").unwrap();
            let type_int = type_interaction.as_u64().unwrap();
            if type_int == HTTP_INTERACTION_CONFIRMATION_BOT {
                return warp::reply::with_status(warp::reply::json(&json!({ "type": 1 }).as_object_mut()), warp::http::StatusCode::OK);
            }
            if type_int == INTERACTION_COMMAND {
                return warp::reply::with_status(warp::reply::json(&json!({ "type": 5 }).as_object_mut()), warp::http::StatusCode::OK);
            }

            return warp::reply::with_status(warp::reply::json(&json!({ "status_code": 200, "message": "Interaction unknown or not recognized", "error": false, "code_error": "HTTP_INTERACTION_UKNOWN" }).as_object_mut()), warp::http::StatusCode::OK)
        }
        false => {
           return warp::reply::with_status(warp::reply::json(&json!({ "status_code": 401, "message": "Uh! It appears that this signature or metadata is incorrect. Check it out: https://discord.com/developers/docs/interactions/receiving-and-responding", "error": true, "code": "HTTP_UNAUTHORIZED" }).as_object_mut()), warp::http::StatusCode::UNAUTHORIZED)
        }
    }
}