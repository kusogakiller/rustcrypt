use wasm_bindgen::prelude::*;
use rand::RngCore;
use blake3;

#[wasm_bindgen]
pub fn generate(size: usize) -> String {
    let mut input = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut input);

    let mut output = vec![0u8; size];

    blake3::Hasher::new()
        .update(&input)
        .finalize_xof()
        .fill(&mut output);

    hex::encode(output)
}
