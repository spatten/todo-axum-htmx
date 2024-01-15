use axum::http::StatusCode;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use sqlx::PgPool;
use std::num::NonZeroU32;
use tower_cookies::cookie::SameSite;
use tower_cookies::cookie::{
    time::{Duration, OffsetDateTime},
    Key,
};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use crate::utils;

use super::db;
use super::templates::UserForm;

static COOKIE_NAME: &str = "SESSION";

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

impl User {
    // Authenticate the given user model with the password
    pub fn authenticate(&self, password: &str) -> bool {
        let n_iter = NonZeroU32::new(100_000).unwrap();
        pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            self.salt.as_bytes(),
            password.as_bytes(),
            self.password_hash.as_bytes(),
        )
        .is_ok()
    }

    // Set login cookie
    pub fn set_cookie(&self, cookies: Cookies) -> Result<(), (StatusCode, String)> {
        println!("start of set cookie");
        // Build the cookie, and make it private
        let password_slice = self.password_hash.get(0..29);
        let Some(password_slice) = password_slice else {
            return Err(utils::internal_error_from_string("failed to set cookie"));
        };
        let Some(id) = self.id else {
            return Err(utils::internal_error_from_string("user has no ID"));
        };
        // generated with `Key::generate()` and then printed with `HEXUPPER.encode(key.master())`
        // let key = Key::generate();
        // prinln!("Key: '{:?}'", HEXUPPER.encode(key.master()));
        // let key = Key::from(&[
        //     30, 93, 127, 174, 100, 98, 218, 101, 171, 145, 32, 39, 254, 146, 254, 248, 21, 237, 27,
        //     211, 102, 103, 241, 58, 190, 0, 104, 227, 76, 96, 105, 207, 35, 211, 58, 19, 179, 130,
        //     153, 250, 179, 195, 44, 51, 243, 60, 4, 0, 127, 208, 90, 102, 28, 20, 169, 169, 142, 5,
        //     39, 56, 152, 72, 193, 189,
        // ]);
        let key = HEXUPPER.decode("1E5D7FAE6462DA65AB912027FE92FEF815ED1BD36667F13ABE0068E34C6069CF23D33A13B38299FAB3C32C33F33C04007FD05A661C14A9A98E0527389848C1BD".as_bytes()).map_err(utils::internal_error)?;
        let key = Key::from(&key);
        println!("key: '{:?}'", HEXUPPER.encode(key.master()));
        let private = cookies.private(&key);
        let user_key = format!("{}----{}", id, password_slice);
        let now = OffsetDateTime::now_utc();
        let three_months = Duration::days(90);

        let cookie = Cookie::build((COOKIE_NAME, user_key))
            .path("/")
            .secure(true)
            .expires(now + three_months)
            .same_site(SameSite::Strict)
            .http_only(true)
            .into();
        private.add(cookie);
        println!("logged in. Redirecting to /");

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
