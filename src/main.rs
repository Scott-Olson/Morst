#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use mors::{decoder, encoder};

#[get("/")]
fn index() -> &'static str {
    let encoder = encoder::Encoder::new();
    let decoder = decoder::Decoder::new();
    let s = String::from("morst").to_uppercase();

    let result_encode = encoder.encode_letters(s);

    let message = String::from("-- --- .-. ... -");
    let result_decode = decoder.decode_message(message);
    "Encode: {result_encode}, Decode: {result_decode}"
}

fn main() {
    println!(" -=-=-=-=-=-=- Main Loop -=-=-=-=-=-=- ");
    let s = String::from("morst").to_uppercase();

    let morse_encoder = encoder::Encoder::new();
    let result_encode = morse_encoder.encode_letters(s);

    let decoder = decoder::Decoder::new();
    let message = String::from("-- --- .-. ... -");
    let result_decode = decoder.decode_message(message);
    println!("Encode: {0}, Decode: {1} ", result_encode, result_decode);

    rocket::ignite().mount("/", routes![index]).launch();
}
