/// Image Processing Module
/// Provides functionalities to process an image with user-defined or default brightness and contrast
mod errors;
use errors::ImageProcessingError;

use opencv::{
    core::{self, MatTrait},
    imgcodecs,
};
use serde::Deserialize;
use std::env;
use warp::Filter;

/// Default brightness value
const DEFAULT_BRIGHTNESS: f64 = 0.5;
/// Default contrast value
const DEFAULT_CONTRAST: f64 = 1.0;

/// Holds the optional brightness and contrast values for image processing
///
/// * `brightness` - An optional f64 value for brightness
/// * `contrast` - An optional f64 value for contrast
#[derive(Debug, Deserialize)]
struct ImageProcessingOptions {
    brightness: Option<f64>,
    contrast: Option<f64>,
}

/// Reads the command line arguments, sets up the server route and starts the server
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Usage: {} <path-to-image>",
            args.get(0).unwrap_or(&"app".to_string())
        );
        return;
    }

    let image_path = args[1].clone();

    // Starting the server on localhost and port 8080
    warp::serve(create_route(image_path))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

/// Creates a server route that listens to the "/image" path and processes incoming requests
///
/// # Arguments
/// * `image_path` - A String that holds the path to the image file
fn create_route(
    image_path: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("image")
        .and(warp::query::<ImageProcessingOptions>())
        .and(warp::any().map(move || image_path.clone()))
        .and_then(process_image)
}

/// Asynchronously processes the image based on provided options and returns the processed image
///
/// # Arguments
/// * `options` - The image processing options containing user-defined or default brightness and contrast values
/// * `image_path` - A String that holds the path to the image file
async fn process_image(
    options: ImageProcessingOptions,
    image_path: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    validate_options(&options)?;

    let src = imgcodecs::imread(&image_path, imgcodecs::IMREAD_COLOR)
        .map_err(|_| warp::reject::custom(ImageProcessingError::ImageReadError))?;

    let alpha = options.contrast.unwrap_or(DEFAULT_CONTRAST); // contrast: [0.0, 1.0]

    // We need to beta be zero when brightness is 0.5. Also brightness limits are [0.0, +inf),
    // so it is why we use log2(brightness / 0.5). It gives us -inf when brightness approaches 0 and
    // +inf when brightness approaches +inf
    let beta = (options.brightness.unwrap_or(DEFAULT_BRIGHTNESS) / 0.5).log2(); // brightness: [0.0, +inf)

    let mut dst = src.clone();

    if src.convert_to(&mut dst, -1, alpha, beta).is_err() {
        return Err(warp::reject::custom(
            ImageProcessingError::ImageConversionError,
        ));
    }

    let mut buf = core::Vector::<u8>::new();
    if imgcodecs::imencode(".jpeg", &dst, &mut buf, &core::Vector::new()).is_err() {
        return Err(warp::reject::custom(
            ImageProcessingError::ImageEncodingError,
        ));
    }

    Ok(warp::reply::with_header(
        buf.to_vec(),
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
