extern crate base64;
extern crate hex;
extern crate rustc_serialize;

use rustc_serialize::hex::FromHex;

fn convert_data_for_byte(string_data: &str) -> Vec<u8> {
   return string_data.from_hex().unwrap()
}

pub fn decode_data() {}