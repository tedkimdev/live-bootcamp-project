use std::error::Error;

use crate::domain::{User, UserStore, UserStoreError};
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use sqlx::PgPool;

use crate::domain::Email;
use crate::domain::Password;

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let password_hash = compute_password_hash(user.password.0.to_string()).await;
        if password_hash.is_err() {
            return Err(UserStoreError::UnexpectedError);
        }

        let _ = sqlx::query!(
            r#"
            INSERT INTO public.users
            (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#,
            user.email.as_ref(),
            password_hash.unwrap(),
            user.require_2fa,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_error) if db_error.constraint() == Some("users_email_key") => {
                UserStoreError::UserAlreadyExists
            }
            _ => UserStoreError::UnexpectedError,
        });

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let maybe_user = sqlx::query!("SELECT * from users where email = $1", email.as_ref(),)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        match maybe_user {
            Some(record) => {
                let email = Email::parse(record.email).unwrap();
                let password = Password::parse(record.password_hash).unwrap();
                let require_2fa = record.requires_2fa;
                Ok(User::new(email, password, require_2fa))
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let maybe_user = sqlx::query!("SELECT * from users where email = $1", email.as_ref(),)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        if let Some(user) = maybe_user {
            let verified = verify_password_hash(user.password_hash, password.0.clone())
                .await
                .map_err(|_e| UserStoreError::InvalidCredentials);
            return verified;
        }

        Err(UserStoreError::InvalidCredentials)
    }

    async fn delete_user(
        &mut self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let maybe_user = sqlx::query!("SELECT * from users where email = $1", email.as_ref(),)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError);

        if maybe_user.is_err() {
            return Err(UserStoreError::UnexpectedError);
        }
        if maybe_user.is_ok() && maybe_user.as_ref().unwrap().is_none() {
            return Err(UserStoreError::UserNotFound);
        }

        if let Some(user) = maybe_user.unwrap() {
            let verified = verify_password_hash(user.password_hash, password.0.clone())
                .await
                .map_err(|_e| UserStoreError::InvalidCredentials);

            if verified.is_err() {
                return Err(UserStoreError::InvalidCredentials);
            }
        }

        let _ = sqlx::query!(
            r#"
            DELETE FROM public.users
            WHERE email = $1
            "#,
            email.as_ref(),
        )
        .execute(&self.pool)
        .await
        .map_err(|_e| UserStoreError::UnexpectedError);

        Ok(())
    }
}

// Helper function to verify if a given password matches an expected hash
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error>> {
    let result = tokio::task::spawn_blocking(move || {
        let expected_password_hash: PasswordHash<'_> =
            PasswordHash::new(expected_password_hash.as_str())?;
        Argon2::default().verify_password(password_candidate.as_bytes(), &expected_password_hash)
    })
    .await?;

    result.map_err(|e| e.into())
}

// Helper function to hash passwords before persisting them in the database.
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking.
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let password_hash: Result<String, argon2::password_hash::Error> = tokio::task::spawn_blocking(move || {
        let salt: SaltString = SaltString::generate(&mut rand::rngs::OsRng);
        Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(password.as_bytes(), &salt)
        .map(|password_hash| password_hash.to_string())
    })
    .await?;

    match password_hash {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::new(e)),
    }
}
