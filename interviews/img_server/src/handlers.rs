use crate::errors::ImageProcessingError;
use crate::image_processing::{
    process_image, ImageProcessingOptions, DEFAULT_BRIGHTNESS, DEFAULT_CONTRAST,
};
use warp::Rejection;

pub async fn process_image_handler(
    options: ImageProcessingOptions,
    image_path: String,
) -> Result<impl warp::Reply, Rejection> {
    validate_options(&options)?;

    let processed_image = match process_image(
        &image_path,
        options.brightness.unwrap_or(DEFAULT_BRIGHTNESS),
        options.contrast.unwrap_or(DEFAULT_CONTRAST),
    ) {
        Ok(image) => image,
        Err(_err) => {
            return Err(warp::reject::custom(
                ImageProcessingError::ImageConversionError,
            ))
        }
    };

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
