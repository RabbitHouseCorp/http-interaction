// Cryptography is used to decrypt the metadata that Discord provides during an API request.
// You can verify information through the Discord documentation.
//
// https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization
//
// Metadata we collect is:
// Interaction Structures: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure
//
// Any issues related to encryption you can contact me on my Github (https://github.com/nayvcake).
// I have faith that we use cryptography for good purposes. We don't use cryptography for bad faith stuff.
// This code follows the terms of use of the social platform ("Discord").

// Data
//
// We do not save the data nor transmit it to such third party services.
// We use encryption to protect data through packet service which is Websocket.
//
//
// Update: .==AjMtMDM5ITOwETO40ych52bpp2chVHZ352bppWdh9WaqV3.cu9mbvRma35WYrpmbstmak5GbuNXYrpmbklmaupmbhx2at5GbrNXYt52bq/.RmbvtmapdXctFHcvtGZzF2asZGZzFWY==-.

extern crate ed25519_dalek;
use self::ed25519_dalek::{PublicKey, Verifier};
use self::ed25519_dalek::ed25519::signature::Signature;


// These are Discord's encryption signature.
//
// pub const SIGNATURE_DISCORD: usize = 0x00000020;
// pub const SIGNATURE_DISCORD_TIMESTAMP: usize = 0x00000040;
pub fn verify_authorization(pub_key: String, sign: String, message: String) -> bool {
    let public_key = hex::decode(pub_key);
    if public_key.is_err() { return false; }
    let hex_signature = hex::decode(sign);
    if hex_signature.is_err() {  return false; }
    let public_key = PublicKey::from_bytes(&public_key.unwrap());
    if public_key.is_err() {  return false; }
    let signature = ed25519_dalek::Signature::from_bytes(&hex_signature.unwrap());
    if signature.is_err() {  return false; }
    return public_key.unwrap().verify(message.as_bytes(), &signature.unwrap()).is_ok()
}

