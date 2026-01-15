use crate::{errors::MyError, models::User};

/// Check if the user has the 'admin' role
pub fn require_admin(user: &User) -> Result<(), MyError> {
    if user.user_role != "admin" {
        return Err(MyError::Unauthorized);
    }
    Ok(())
}

/// Check if the user is admin or accessing their own resource
pub fn require_admin_or_self(user: &User, target_id: i32) -> Result<(), MyError> {
    if user.user_role == "admin" || user.user_id == target_id {
        return Ok(());
    }
    Err(MyError::Unauthorized)
}

/// Helper alias for readability
pub fn can_modify_user(user: &User, target_id: i32) -> Result<(), MyError> {
    require_admin_or_self(user, target_id)
}
