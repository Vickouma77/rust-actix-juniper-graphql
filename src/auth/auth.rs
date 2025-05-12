use juniper::{FieldResult, graphql_value};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use log::{info, error};
use serde::{Serialize, Deserialize};
use crate::context::Context;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
}

pub fn login(context: &Context, email: String) -> FieldResult<String> {
    info!("Login attempt for email: {}", email);
    let user = context
        .users
        .iter()
        .find(|u| u.email == email)
        .ok_or_else(|| {
            error!("Login failed: User not found for email: {}", email);
            juniper::FieldError::new("User not found", graphql_value!({}))
        })?;
    let claims = Claims { sub: user.id.clone() };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("my-secret-key".as_ref()),
    )?;
    info!("Login successful for user: {}", user.id);
    Ok(token)
}

pub fn verify_token(token: &str) -> Option<String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("my-secret-key".as_ref()),
        &Validation::default(),
    )
    .ok()
    .map(|data| data.claims.sub)
}