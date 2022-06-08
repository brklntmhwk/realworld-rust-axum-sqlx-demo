use argon2::Config;

use conduit_core::config::ConduitConfig;
use conduit_core::errors::{ConduitError, ConduitResult};
use conduit_core::services::security_service::SecurityService;

pub struct ArgonSecurityService {
    config: ConduitConfig,
}

impl ArgonSecurityService {
    pub fn new(config: ConduitConfig) -> Self {
        Self { config }
    }
}

impl SecurityService for ArgonSecurityService {
    fn hash_password(&self, raw_password: &str) -> ConduitResult<String> {
        let password_bytes = raw_password.as_bytes();
        let hashed_password = argon2::hash_encoded(
            password_bytes,
            self.config.argon_salt.as_bytes(),
            &Config::default(),
        )
        .unwrap();

        Ok(hashed_password)
    }

    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: &str,
    ) -> ConduitResult<bool> {
        let hashes_match =
            argon2::verify_encoded(stored_password, attempted_password.as_bytes())
                .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(hashes_match)
    }
}