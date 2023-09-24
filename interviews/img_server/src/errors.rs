/// Image Processing Error Module
/// Provides a custom error type, `ImageProcessingError`
use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum ImageProcessingError {
    /// Represents errors related to invalid parameters
    #[error("Invalid Parameters: {0}")]
    InvalidParameters(String),

    /// Represents errors that can occur while reading an image
    #[error("Error reading the image")]
    ImageReadError,

    /// Represents errors that can occur while converting the image
    #[error("Error converting the image")]
    ImageConversionError,

    /// Represents errors that can occur while encoding the image
    #[error("Error encoding the image")]
    ImageEncodingError,
}

impl Reject for ImageProcessingError {}
