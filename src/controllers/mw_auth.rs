use crate::controllers::AUTH_TOKEN;
use crate::ctx::Ctx;
use crate::models::message::Claims;
use crate::{CustomError, Result};
use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, Request};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use super::{BEARER, JWT_SECRET};

pub async fn mw_ctx_resolver(mut req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    // Compute Result<Ctx>.
    let result_ctx: Ctx = match authorize(req.headers()) {
        Ok(ctx) => ctx,
        Err(e) => return Err(e),
    };

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTH_TOKEN) {
        Some(v) => v,
        None => return Err(CustomError::AuthFailNoAuthToken),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(CustomError::AuthFailNoAuthToken),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(CustomError::AuthFailNoAuthToken);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

fn authorize(headers: &HeaderMap<HeaderValue>) -> Result<Ctx> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| CustomError::AuthFailNoAuthToken)?;

            Ok(Ctx {
                email: decoded.claims.email,
                token: jwt,
            })
        }
        Err(e) => Err(e),
    }
}
