use std::num::NonZeroU32;

use axum::http::StatusCode;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use sqlx::PgPool;

use super::db;
use super::templates::UserForm;

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
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

impl UserForm {
    pub async fn validate(mut self, pool: &PgPool) -> Result<Self, (StatusCode, String)> {
        // password validations
        let mut password_errors = vec![];
        if self.password.len() < 10 {
            password_errors.push("passwords must be at least 10 characters long".to_string())
        }
        if self.password != self.password_confirmation {
            password_errors.push("password and password confirmation must match".to_string())
        };
        self.password_errors = password_errors.join(", ");

        // email validations
        let mut email_errors = vec![];
        let existing = db::find_by_email(self.email.clone(), pool).await?;
        if existing.is_some() {
            email_errors.push("A user with this email already exists".to_string())
        }
        self.email_errors = email_errors.join(", ");
        Ok(self)
    }

    pub fn is_valid(&self) -> bool {
        self.password_errors.is_empty() && self.email_errors.is_empty()
    }
}

impl TryFrom<UserForm> for User {
    type Error = UserForm;
    fn try_from(form: UserForm) -> Result<User, UserForm> {
        Ok(User {
            email: form.email,
            password_hash: form.password,
            salt: "1234".to_string(),
            id: None,
        })
    }
}
