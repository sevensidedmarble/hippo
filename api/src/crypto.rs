use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(bytes: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2.hash_password(bytes, &salt).unwrap().to_string()
}

pub fn verify_password(password: &[u8], password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok()
}
