use anyhow::Error as AnyhowError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GUIError {
    #[error("An anyhow error occurred: {0}")]
    Anyhow(#[from] AnyhowError),
    // Add other error variants as needed
}

impl serde::Serialize for GUIError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type GUIResult<T> = Result<T, GUIError>;