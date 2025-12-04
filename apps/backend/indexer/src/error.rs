use std::fmt::Debug;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing `{0}` environment variable")]
    MissingEnvVar(String),

    #[error("Invalid ChainID: `{0}`")]
    InvalidChainID(String),
}

#[cfg(test)]
mod tests {
    use super::AppError;

    #[test]
    fn missing_env_var_displays_message() {
        let err = AppError::MissingEnvVar("DATABASE_URL".into());
        assert_eq!(
            format!("{}", err),
            "Missing `DATABASE_URL` environment variable"
        );
    }

    #[test]
    fn invalid_chain_id_displays_message() {
        let err = AppError::InvalidChainID("999".into());
        assert_eq!(format!("{}", err), "Invalid ChainID: `999`");
    }
}
