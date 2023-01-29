use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum GeneralErrors {
    #[error("Unable to read configureation")]
    ReadConfigError,
    #[error("Error during tcp listener creation")]
    TcpListenerError,
}
