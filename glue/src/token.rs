#[cfg(feature = "actix")]
use actix_web::{dev::Payload, FromRequest, HttpRequest};

#[cfg(feature = "actix")]
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[cfg(feature = "actix")]
use crate::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderToken {
    pub unique_id: String,
}

impl HeaderToken {
    pub fn get_key() -> Result<String, NanoServiceError> {
        std::env::var("JWT_SECRET")
            .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unauthorized))
    }
    pub fn encode(self) -> Result<String, NanoServiceError> {
        let key_str = Self::get_key()?;
        let key = EncodingKey::from_secret(key_str.as_ref());
        match encode(&Header::default(), &self, &key) {
            Ok(token) => Ok(token),
            Err(error) => Err(NanoServiceError::new(
                error.to_string(),
                NanoServiceErrorStatus::Unauthorized,
            )),
        }
    }
    pub fn decode(token: &str) -> Result<Self, NanoServiceError> {
        let key_str = Self::get_key()?;
        let key = DecodingKey::from_secret(key_str.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.required_spec_claims.remove("exp");

        match decode::<Self>(token, &key, &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(error) => Err(NanoServiceError::new(
                error.to_string(),
                NanoServiceErrorStatus::Unauthorized,
            )),
        }
    }
}

#[cfg(feature = "actix")]
impl FromRequest for HeaderToken {
    type Error = NanoServiceError;

    type Future = Ready<Result<HeaderToken, NanoServiceError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let raw_data = match req.headers().get("token") {
            Some(data) => data.to_str().expect("convert token to str"),
            None => {
                return err(NanoServiceError {
                    status: NanoServiceErrorStatus::Unauthorized,
                    message: "token not in header under key 'token'".to_string(),
                });
            }
        };

        let token = match HeaderToken::decode(raw_data) {
            Ok(token) => token,
            Err(_) => {
                return err(NanoServiceError {
                    status: NanoServiceErrorStatus::Unauthorized,
                    message: "token not a valid string".to_string(),
                });
            }
        };
        ok(token)
    }
}
