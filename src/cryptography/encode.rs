// ed25519_dalek::Signature
extern crate base64;
extern crate hex;
extern crate rustc_serialize;
extern crate ed25519_dalek;
extern crate crypto;
extern crate rand;
use rand::rngs::OsRng;
use std::borrow::Borrow;
use ed25519_dalek::{Digest, Keypair, SecretKey, Sha512, SignatureError};
use hex::ToHex;
use rustc_serialize::hex::FromHex;
use serde::de::Unexpected::Str;
use self::ed25519_dalek::{PublicKey, Verifier};
use self::ed25519_dalek::ed25519::signature::Signature;
use self::ed25519_dalek::{Signer};
fn convert_data_for_byte(string_data: &str) -> Vec<u8> {
    return string_data.from_hex().unwrap()
}

pub fn encode_data(key_secret: String, message: String) -> (String, Sha512, Result<SecretKey, SignatureError>) {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let key = ed25519_dalek::SecretKey::from_bytes(key_secret.as_bytes());
    let sha = ed25519_dalek::Sha512::new();
    let signature = keypair.sign(message.as_ref());
    return (base64::encode(signature.to_string()), sha, key)
}
