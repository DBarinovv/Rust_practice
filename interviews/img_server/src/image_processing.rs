/// Image Processing Module
/// Provides a core logic for image processing
use crate::errors::ImageProcessingError;
use opencv::{
    core, imgcodecs,
    prelude::{Mat, MatTrait},
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

/// Default brightness value
pub const DEFAULT_BRIGHTNESS: f64 = 0.5;
/// Default contrast value
pub const DEFAULT_CONTRAST: f64 = 1.0;

/// Holds the optional brightness and contrast values for image processing
///
/// * `brightness` - An optional f64 value for brightness
/// * `contrast` - An optional f64 value for contrast
#[derive(Debug, Deserialize)]
pub struct ImageProcessingOptions {
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
}

/// Processes an image with the given brightness and contrast values and returns the processed image
///
/// # Arguments
/// * `image_src` - The source image wrapped in an Arc<Mutex<Mat>>
/// * `brightness` - A f64 value for adjusting the brightness of the image
/// * `contrast` - A f64 value for adjusting the contrast of the image
pub fn process_image(
    image_src: Arc<Mutex<Mat>>,
    brightness: f64,
    contrast: f64,
) -> Result<Vec<u8>, ImageProcessingError> {
    let alpha = contrast; // contrast: [0.0, 1.0]

    // We need to beta be zero when brightness is 0.5. Also brightness limits are [0.0, +inf),
    // so it is why we use log2(brightness / 0.5). It gives us -inf when brightness approaches 0 and
    // +inf when brightness approaches +inf
    let beta = (brightness / 0.5).log2(); // brightness: [0.0, +inf)

    let locked_src = image_src.lock().unwrap();
    let mut dst = locked_src.clone();

    if locked_src.convert_to(&mut dst, -1, alpha, beta).is_err() {
        return Err(ImageProcessingError::ImageConversionError);
    }

    // Mutex release
    drop(locked_src);

    let mut buf = core::Vector::<u8>::new();
    if imgcodecs::imencode(".jpeg", &dst, &mut buf, &core::Vector::new()).is_err() {
        return Err(ImageProcessingError::ImageEncodingError);
    }

    Ok(buf.to_vec())
}
