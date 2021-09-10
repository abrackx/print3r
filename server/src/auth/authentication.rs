use rand;
use rand::Rng;
use regex::Regex;
use base64;
use sha2::{Sha256, Digest};

pub fn generate_verifier() -> String {
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
    return base64::encode_config(random_bytes, base64::URL_SAFE_NO_PAD);
}

pub fn generate_challenge(verifier: String) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    return base64::encode_config(&hash, base64::URL_SAFE_NO_PAD);
}

pub fn get_auth_redirect_url(challenge: String) -> String {
    return format!("https://dev-05tizgpa.us.auth0.com/authorize?\
    response_type=code&\
    code_challenge={}&\
    code_challenge_method=S256&\
    client_id=xodFBsdfd2LQXzzaqac3979dnE8GhcEq&\
    redirect_uri=http://localhost:8888/api/v1/auth0_callback&\
    scope=openid profile email&\
    audience=https://dev-05tizgpa.us.auth0.com/api/v2/", challenge).to_string()
}

