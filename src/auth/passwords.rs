use argonautica::{Error, Hasher, Verifier};
use lazy_static::lazy_static;

use crate::constants::messages;

lazy_static! {
    static ref PASSWORD_SECRET_KEY: String = std::env::var("PASSWORD_SECRET_KEY")
        .expect(messages::MESSAGE_INVALID_PASSWORD_SECRET_KEY.into());
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .hash()
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .verify()
}
