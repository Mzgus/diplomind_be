// Unit tests for authentication functions (password hashing)

use diplomind::services::auth;

#[test]
fn test_hash_password_generates_valid_hash() {
    let password = "TestPassword123";
    let hash = auth::hash_password(password).unwrap();
    
    // Verify hash starts with $argon2
    assert!(hash.starts_with("$argon2"));
    
    // Verify hash is not the plain password
    assert_ne!(hash, password);
    
    // Verify hash has reasonable length (Argon2 hashes are ~90+ chars)
    assert!(hash.len() > 80);
}

#[test]
fn test_verify_password_with_correct_password() {
    let password = "TestPassword123";
    let hash = auth::hash_password(password).unwrap();
    
    let is_valid = auth::verify_password(password, &hash).unwrap();
    assert!(is_valid);
}

#[test]
fn test_verify_password_with_wrong_password() {
    let password = "TestPassword123";
    let hash = auth::hash_password(password).unwrap();
    
    let is_valid = auth::verify_password("WrongPassword", &hash).unwrap();
    assert!(!is_valid);
}

#[test]
fn test_hash_password_different_salts() {
    let password = "TestPassword123";
    let hash1 = auth::hash_password(password).unwrap();
    let hash2 = auth::hash_password(password).unwrap();
    
    // Same password should produce different hashes (different salts)
    assert_ne!(hash1, hash2);
    
    // But both should verify correctly
    assert!(auth::verify_password(password, &hash1).unwrap());
    assert!(auth::verify_password(password, &hash2).unwrap());
}

#[test]
fn test_verify_password_with_empty_password() {
    let password = "TestPassword123";
    let hash = auth::hash_password(password).unwrap();
    
    let is_valid = auth::verify_password("", &hash).unwrap();
    assert!(!is_valid);
}

#[test]
fn test_hash_password_with_special_characters() {
    let password = "P@ssw0rd!#$%^&*()";
    let hash = auth::hash_password(password).unwrap();
    
    assert!(hash.starts_with("$argon2"));
    assert!(auth::verify_password(password, &hash).unwrap());
}

#[test]
fn test_hash_password_with_unicode() {
    let password = "Pässwörd123";
    let hash = auth::hash_password(password).unwrap();
    
    assert!(hash.starts_with("$argon2"));
    assert!(auth::verify_password(password, &hash).unwrap());
}
