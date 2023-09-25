/// Image Processing Module
/// Provides functionalities to process an image with user-defined or default brightness and contrast
mod errors;
mod handlers;
mod image_processing;

use errors::ImageProcessingError;
use handlers::process_image_handler;

use image_processing::ImageProcessingOptions;
use opencv::{imgcodecs, prelude::Mat};
use std::{
    env,
    sync::{Arc, Mutex},
};
use warp::Filter;

/// Reads the command line arguments, sets up the server route and starts the server
#[tokio::main]
async fn main() -> Result<(), ImageProcessingError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Usage: {} <path-to-image>",
            args.get(0).unwrap_or(&"app".to_string())
        );

        return Err(ImageProcessingError::InvalidParameters(
            "Incorrect number of arguments".to_string(),
        ));
    }

    let image_path = args[1].clone();

    // One read and decode for all program
    let image_src = imgcodecs::imread(&image_path, imgcodecs::IMREAD_COLOR)
        .map_err(|_| ImageProcessingError::ImageReadError)?;

    let image_src = Arc::new(Mutex::new(image_src));

    // Starting the server on localhost and port 8080
    warp::serve(create_route(image_src))
        .run(([127, 0, 0, 1], 8080))
        .await;

    Ok(())
}

/// Creates a server route that listens to the "/image" path and processes incoming requests
///
/// # Arguments
/// * `image_path` - A String that holds the path to the image file
fn create_route(
    image_src: Arc<Mutex<Mat>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("image")
        .and(warp::query::<ImageProcessingOptions>())
        .and(warp::any().map(move || Arc::clone(&image_src)))
        .and_then(process_image_handler)
}
