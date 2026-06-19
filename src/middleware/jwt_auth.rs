use crate::{errors::MyError, models, services::auth::TokenManager};
use jsonwebtoken::{DecodingKey, Validation, decode};
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};

/// Middleware to verify JWT tokens on protected routes
pub struct JwtAuth {
    token_manager: TokenManager,
}

impl JwtAuth {
    pub fn new(token_manager: TokenManager) -> Self {
        Self { token_manager }
    }
}

impl<E: Endpoint> Middleware<E> for JwtAuth {
    type Output = JwtAuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        JwtAuthEndpoint {
            ep,
            token_manager: self.token_manager.clone(),
        }
    }
}

pub struct JwtAuthEndpoint<E> {
    ep: E,
    token_manager: TokenManager,
}

impl<E: Endpoint> Endpoint for JwtAuthEndpoint<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        // Extract Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(MyError::TokenExpired)?; // Use our custom error

        // Extract token from "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(MyError::TokenExpired)?; // Use our custom error

        // Decode and validate the JWT
        let token_data = decode::<models::JWTClaims>(
            token,
            &DecodingKey::from_secret(self.token_manager.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| MyError::TokenExpired)?; // Use our custom error

        // Check if user is active (not deactivated by admin)
        if !token_data.claims.user.user_active {
            return Err(MyError::Unauthorized.into());
        }

        // Store the authenticated user in request extensions
        req.extensions_mut().insert(token_data.claims.user);

        // Continue to the endpoint
        let res = self.ep.call(req).await;

        match res {
            Ok(resp) => Ok(resp.into_response()),
            Err(err) => Err(err),
        }
    }
}

/// Extractor for authenticated user
/// Use this in your handlers to get the authenticated user
pub struct AuthUser(pub models::User);

impl<'a> poem::FromRequest<'a> for AuthUser {
    async fn from_request(req: &'a Request, _body: &mut poem::RequestBody) -> Result<Self> {
        let user = req
            .extensions()
            .get::<models::User>()
            .cloned()
            .ok_or(MyError::TokenExpired)?; // Use our custom error

        Ok(AuthUser(user))
    }
}
