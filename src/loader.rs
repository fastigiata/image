use napi::{Error, Result};
use crate::core::ImageWrapper;

/// Image loader provides several way to load images into ImageWrapper
#[napi]
pub struct ImageLoader {}

#[napi]
impl ImageLoader {
    /// Create a new image from a byte slice. Makes an educated guess about the image format
    #[napi]
    pub fn auto_guess(buffer: Vec<u8>) -> Result<ImageWrapper> {
        match image::load_from_memory(&buffer) {
            Ok(img) => Ok(ImageWrapper::new(img)),
            Err(err) => Err(Error::from_reason(format!("failed to load image: {}", err)))
        }
    }

    /// Create a new image from a byte slice. Assumes the image is a PNG
    #[napi]
    pub fn from_png(buffer: Vec<u8>) -> Result<ImageWrapper> {
        match image::load_from_memory_with_format(&buffer, image::ImageFormat::Png) {
            Ok(img) => Ok(ImageWrapper::new(img)),
            Err(err) => Err(Error::from_reason(format!("failed to load image: {}", err)))
        }
    }

    // TODO: other format
}