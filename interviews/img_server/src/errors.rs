/// Image Processing Error Module
/// Provides a custom error type, `ImageProcessingError`
use std::fmt;
use warp::reject::Reject;

/// Represents various errors that can occur during the image processing
///
/// * `InvalidParameters(String)`: Represents errors related to invalid parameters, holds a String with a descriptive error message
/// * `ImageReadError`: Represents errors that can occur while reading an image
/// * `ImageConversionError`: Represents errors that can occur while converting the image, for example during color adjustments
/// * `ImageEncodingError`: Represents errors that can occur while encoding the image, for example, converting it to a specific format
#[derive(Debug)]
pub enum ImageProcessingError {
    InvalidParameters(String),
    ImageReadError,
    ImageConversionError,
    ImageEncodingError,
}

impl Reject for ImageProcessingError {}

impl fmt::Display for ImageProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageProcessingError::InvalidParameters(msg) => {
                write!(f, "Invalid Parameters: {}", msg)
            }
            ImageProcessingError::ImageReadError => write!(f, "Error reading the image"),
            ImageProcessingError::ImageConversionError => write!(f, "Error converting the image"),
            ImageProcessingError::ImageEncodingError => write!(f, "Error encoding the image"),
        }
    }
}
