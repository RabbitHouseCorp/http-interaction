// https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization
//
// Using interaction structures: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure

extern crate ed25519_dalek;

use self::ed25519_dalek::{PublicKey, Verifier};
use tracing::log::error;

pub fn verify_authorization(pub_key: String, sign: String, message: String) -> bool {
    let public_key = hex::decode(pub_key);
    if public_key.is_err() {
        error!("err -> Err: public_key_decode");
        return false;
    }
    let hex_signature = hex::decode(sign);
    if hex_signature.is_err() {
        error!("err -> Err: hex_signature");
        return false;
    }
    let public_key = PublicKey::from_bytes(&public_key.unwrap().clone());
    if public_key.is_err() {
        error!("err -> Err: public_key");
        return false;
    }
    let signature = ed25519_dalek::Signature::from_bytes(&hex_signature.unwrap());
    if signature.is_err() {
        // eprintln!("err -> Err: signature");
        return false;
    }
    let ok = public_key
        .unwrap()
        .verify(message.as_bytes(), &signature.unwrap());
    if ok.is_err() {
        return false;
    }
    return true;
}
