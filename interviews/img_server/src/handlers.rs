/// Handlers Module
/// Provides HTTP handler functions to process incoming requests.
use crate::errors::ImageProcessingError;
use crate::image_processing::{
    process_image, ImageProcessingOptions, DEFAULT_BRIGHTNESS, DEFAULT_CONTRAST,
};
use opencv::prelude::Mat;
use std::sync::{Arc, Mutex};
use warp::Rejection;

/// Asynchronously handles the image processing requests
///
/// # Arguments
/// * `options` - The image processing options containing user-defined or default brightness and contrast values
/// * `image_src` - The source image wrapped in an Arc<Mutex<Mat>>
pub async fn process_image_handler(
    options: ImageProcessingOptions,
    image_src: Arc<Mutex<Mat>>,
) -> Result<impl warp::Reply, Rejection> {
    validate_options(&options)?;

    let processed_image = process_image(
        image_src,
        options.brightness.unwrap_or(DEFAULT_BRIGHTNESS),
        options.contrast.unwrap_or(DEFAULT_CONTRAST),
    )
    .map_err(|_| warp::reject::custom(ImageProcessingError::ImageConversionError))?;

    Ok(warp::reply::with_header(
        processed_image.to_vec(),
        "Content-Type",
        "image/jpeg",
    ))
}

/// Validates the provided image processing options
///
/// # Arguments
/// * `options` - A reference to the ImageProcessingOptions struct
fn validate_options(options: &ImageProcessingOptions) -> Result<(), warp::Rejection> {
    if options.brightness.unwrap_or(DEFAULT_BRIGHTNESS) < 0.0
        || options.contrast.unwrap_or(DEFAULT_CONTRAST) < 0.0
        || options.contrast.unwrap_or(DEFAULT_CONTRAST) > 1.0
    {
        return Err(warp::reject::custom(
            ImageProcessingError::InvalidParameters("Invalid brightness or contrast".to_string()),
        ));
    }
    Ok(())
}
