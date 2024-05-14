use std::sync::OnceLock;

static ENV_VARS: OnceLock<Vec<EnvVars>> = OnceLock::new();

#[derive(Debug)]
pub enum EnvVars {
    DatabaseUrl(String),
    TokenSecret(String),
    TokenExpiryTimeInSeconds(i64),
}

impl EnvVars {
    pub fn db_url() -> String {
        let env_vars = ENV_VARS.get().expect("env vars not set");
        env_vars
            .iter()
            .find_map(|env_var| match env_var {
                EnvVars::DatabaseUrl(db_url) => Some(db_url.clone()),
                _ => None,
            })
            .expect("DATABASE_URL not set")
    }
    pub fn token_secret() -> String {
        let env_vars = ENV_VARS.get().expect("env vars not set");
        env_vars
            .iter()
            .find_map(|env_var| match env_var {
                EnvVars::TokenSecret(token_secret) => Some(token_secret.clone()),
                _ => None,
            })
            .expect("TOKEN_SECRET not set")
    }
    pub fn token_expiry_time_in_seconds() -> i64 {
        let env_vars = ENV_VARS.get().expect("env vars not set");
        env_vars
            .iter()
            .find_map(|env_var| match env_var {
                EnvVars::TokenExpiryTimeInSeconds(token_expiry_time_in_seconds) => {
                    Some(*token_expiry_time_in_seconds)
                }
                _ => None,
            })
            .expect("TOKEN_EXPIRY_TIME_IN_SECONDS not set")
    }
    pub fn init() {
        dotenvy::dotenv().expect("dotenvy setup failed");
        let mut env_vars = Vec::with_capacity(3);

        let db_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
        env_vars.push(EnvVars::DatabaseUrl(db_url));

        let token_secret =
            std::env::var("TOKEN_SECRET").expect("TOKEN_SECRET environment variable not set");
        env_vars.push(EnvVars::TokenSecret(token_secret));

        let token_expiry_time_in_seconds = std::env::var("TOKEN_EXPIRY_TIME_IN_SECONDS")
            .expect("TOKEN_EXPIRY_TIME_IN_SECONDS environment variable not set")
            .parse::<i64>()
            .expect("TOKEN_EXPIRY_TIME_IN_SECONDS should be a number");
        env_vars.push(EnvVars::TokenExpiryTimeInSeconds(
            token_expiry_time_in_seconds,
        ));
        ENV_VARS.set(env_vars).expect("env vars set failed");
    }
}
