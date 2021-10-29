extern crate rand;
extern crate ed25519_dalek;

use ed25519_dalek::Keypair;
use self::ed25519_dalek::Signer;

pub const SIGNATURE_DISCORD: usize = 0x0000080;

pub fn verify(key: String, sign: String, message: String) -> bool {
  let publickey = Keypair::from_bytes(key.as_ref()).expect("err for decrypt public_key");
  let signature = publickey.sign(sign.as_ref());

  let signature = publickey.verify(message.as_ref(), &signature);

  return signature.is_ok()
  
}