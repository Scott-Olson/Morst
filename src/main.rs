// use std::io;
use mors::{decoder, encoder};

fn main() {
    println!(" -=-=-=-=-=-=- Main Loop -=-=-=-=-=-=- ");
    let s = String::from("test").to_uppercase();

    let morse_encoder = encoder::Encoder::new();
    let result_encode = morse_encoder.encode_letters(s);

    let decoder = decoder::Decoder::new();
    let message = String::from(". - --- -.-");
    let result_decode = decoder.decode_message(message);
    println!("Encode: {0}, Decode: {1} ", result_encode, result_decode);
}
