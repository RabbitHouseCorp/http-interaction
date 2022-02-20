extern crate base64;
extern crate hex;
extern crate rustc_serialize;
extern crate ed25519_dalek;
extern crate crypto;
use rustc_serialize::hex::FromHex;
use self::ed25519_dalek::{PublicKey, Verifier};
use self::ed25519_dalek::ed25519::signature::Signature;
use self::ed25519_dalek::{Signer};
fn convert_data_for_byte(string_data: &str) -> Vec<u8> {
   return string_data.from_hex().unwrap()
}

pub fn decode_data() {

}