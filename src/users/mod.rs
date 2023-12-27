use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};

mod db;
pub mod routes;
mod templates;

#[derive(Debug, Clone, Default)]
struct User {
    id: Option<i32>,
    email: String,
    password_hash: String,
    salt: String,
}

// https://rust-lang-nursery.github.io/rust-cookbook/cryptography/encryption.html
// To verify a password:
// let should_succeed = pbkdf2::verify(
//         pbkdf2::PBKDF2_HMAC_SHA512,
//         n_iter,
//         &salt,
//         password.as_bytes(),
//         &pbkdf2_hash,
//     );
pub fn salted_hash(password: &str) -> Result<(String, String), ring::error::Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let rng = rand::SystemRandom::new();
    let n_iter = NonZeroU32::new(100_000).unwrap();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));
    Ok((HEXUPPER.encode(&salt), HEXUPPER.encode(&pbkdf2_hash)))
}
