use std::fmt;
use warp::reject::Reject;

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
