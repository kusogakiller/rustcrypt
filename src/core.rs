use rand::RngCore;
use blake3;

pub fn generate(len: usize) -> String {
    let mut raw = vec![0u8; 64];
    rand::thread_rng().fill_bytes(&mut raw);

    let hash = blake3::hash(&raw);
    let hex = hash.to_hex().to_string();

    hex[..len].to_string()
}
