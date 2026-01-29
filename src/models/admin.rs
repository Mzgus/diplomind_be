use chrono::{DateTime, Utc};
use serde::Serialize;

/// Response for session revocation
#[derive(Debug, Serialize)]
pub struct RevocationResponse {
    pub success: bool,
    pub tokens_revoked: u64,
    pub revoked_at: DateTime<Utc>,
}
