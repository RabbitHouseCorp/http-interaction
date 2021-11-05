extern crate ed25519_dalek;

use hex::FromHex;
use self::ed25519_dalek::{PublicKey, SignatureError, Verifier};
use self::ed25519_dalek::ed25519::signature::Signature;

pub const SIGNATURE_DISCORD: usize = 0x00000020;
pub const SIGNATURE_DISCORD_TIMESTAMP: usize = 0x00000040;




pub fn verify_authorization(pub_key: String, sign: String, message: String) -> bool {
    let a = pub_key.as_bytes();
    let public_key = hex::decode(pub_key).unwrap();
    let hex_signature = hex::decode(sign).unwrap();


    let public_key = PublicKey::from_bytes(&public_key).unwrap();
    let signature = ed25519_dalek::Signature::from_bytes(&hex_signature).unwrap();

    return public_key.verify(message.as_bytes(), &signature).is_ok()
}