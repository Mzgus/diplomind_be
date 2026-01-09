// Utility to hash a password using Argon2
// Usage: cargo run --bin hash_password <password>

use diplomind::services::auth;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin hash_password <password>");
        std::process::exit(1);
    }
    
    let password = &args[1];
    
    match auth::hash_password(password) {
        Ok(hash) => {
            println!("Password: {}", password);
            println!("Hash: {}", hash);
            println!("\nYou can use this hash in your database migration or .env file");
        }
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            std::process::exit(1);
        }
    }
}
