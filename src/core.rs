use rand::RngCore;
use blake3;

pub fn generate(hex_len: usize) -> String {
    let byte_len = (hex_len + 1) / 2;

    let mut input = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut input);

    let mut output = vec![0u8; byte_len];

    blake3::Hasher::new()
        .update(&input)
        .finalize_xof()
        .fill(&mut output);

    let hex = hex::encode(output);

    hex[..hex_len.min(hex.len())].to_string()
}