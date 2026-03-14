use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::PgPool;

const SEED_USERS: &[(&str, &str)] = &[("a@gmail.com", "1234")];

pub async fn seed_users(db: &PgPool) {
    let argon2 = Argon2::default();
    for (email, password) in SEED_USERS {
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();
        sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(email)
            .bind(hash)
            .execute(db)
            .await
            .expect("Failed to seed user");
    }
}
