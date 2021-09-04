use rand;
use rand::Rng;
use regex::Regex;
use base64;
use sha256;

pub fn generate_verifier() -> String {
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
    return base64::encode_config(random_bytes, base64::URL_SAFE_NO_PAD);
}

pub fn generate_challenge(verifier: String) -> String {
    let hash = sha256::digest(verifier);
    return base64::encode_config(hash, base64::URL_SAFE_NO_PAD);
}