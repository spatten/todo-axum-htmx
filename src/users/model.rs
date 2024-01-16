use axum::http::StatusCode;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use sqlx::PgPool;
use std::env;
use std::num::NonZeroU32;
use tower_cookies::cookie::SameSite;
use tower_cookies::cookie::{
    time::{Duration, OffsetDateTime},
    Key,
};
use tower_cookies::{Cookie, Cookies};

use crate::{utils, SESSION_COOKIE_NAME};

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
    Ok((HEXUPPER.encode(&pbkdf2_hash), HEXUPPER.encode(&salt)))
}

impl User {
    // Authenticate the given user model with the password
    pub fn authenticate(&self, password: &str) -> bool {
        let decoded_salt = HEXUPPER.decode(self.salt.as_bytes()).unwrap();
        let decoded_password = HEXUPPER.decode(self.password_hash.as_bytes()).unwrap();
        let n_iter = NonZeroU32::new(100_000).unwrap();
        pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &decoded_salt,
            password.as_bytes(),
            &decoded_password,
        )
        .is_ok()
    }

    // Set login cookie
    pub fn set_cookie(&self, cookies: Cookies) -> Result<(), (StatusCode, String)> {
        // Build the cookie, and make it private
        let password_slice = self.password_hash.get(0..29);
        let Some(password_slice) = password_slice else {
            return Err(utils::internal_error_from_string("failed to set cookie"));
        };
        let Some(id) = self.id else {
            return Err(utils::internal_error_from_string("user has no ID"));
        };
        // key is generated with `Key::generate()` and then printed with `HEXUPPER.encode(key.master())`
        // let key = Key::generate();
        // println!("Key: '{:?}'", HEXUPPER.encode(key.master()));
        // in dev, set this in the .env file. In production, set it via a normal environment variable
        let key = env::var("COOKIE_ENCRYPTION_KEY").map_err(utils::internal_error)?;
        let key = HEXUPPER
            .decode(key.as_bytes())
            .map_err(utils::internal_error)?;
        let key = Key::from(&key);
        let private = cookies.private(&key);
        let user_key = format!("{}----{}", id, password_slice);
        let now = OffsetDateTime::now_utc();
        let three_months = Duration::days(90);

        let cookie = Cookie::build((SESSION_COOKIE_NAME, user_key))
            .path("/")
            .secure(true)
            .expires(now + three_months)
            .same_site(SameSite::Strict)
            .http_only(true)
            .into();
        private.add(cookie);

        Ok(())
    }
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
