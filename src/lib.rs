use wasm_bindgen::prelude::*;
<<<<<<< HEAD
use blake3;
use hex;
use getrandom::getrandom;

#[wasm_bindgen]
pub fn generate(hex_len: usize) -> String {
    let byte_len = (hex_len + 1) / 2;

    let mut input = [0u8; 32];
    getrandom(&mut input).unwrap();

    let mut output = vec![0u8; byte_len];
=======
use rand::RngCore;
use blake3;

#[wasm_bindgen]
pub fn generate(size: usize) -> String {
    let mut input = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut input);

    let mut output = vec![0u8; size];
>>>>>>> 6b7a9e783517b6ea6968cbca97d098254d38e34b

    blake3::Hasher::new()
        .update(&input)
        .finalize_xof()
        .fill(&mut output);

<<<<<<< HEAD
    let hex = hex::encode(output);

    hex[..hex_len.min(hex.len())].to_string()
}
=======
    hex::encode(output)
}
>>>>>>> 6b7a9e783517b6ea6968cbca97d098254d38e34b
