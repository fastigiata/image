use std::io::Cursor;
use image::{ColorType, DynamicImage, GenericImageView, ImageError, ImageOutputFormat};
use image::imageops::FilterType;

pub struct ImageWrapper {
    /// a wrapper around ImageBuffer that provides dynamic behavior
    dyn_image: DynamicImage,
}

impl ImageWrapper {
    pub fn new(inner: DynamicImage) -> Self {
        Self {
            dyn_image: inner
        }
    }

    /// Get the dimensions of the image, in pixels
    pub fn dimensions(&self) -> (u32, u32) {
        self.dyn_image.dimensions()
    }

    /// Get the color type of the image
    pub fn color(&self) -> ColorType {
        self.dyn_image.color()
    }

    /// Get the number of bits per pixel
    pub fn bits_per_pixel(&self) -> u16 {
        self.dyn_image.color().bits_per_pixel()
    }

    /// Resize this image using the specified filter algorithm. Returns a new image. Does not preserve aspect ratio. nw and nh are the new image's dimensions
    pub fn resize(&mut self, nw: u32, nh: u32, filter: FilterType) -> Self {
        Self {
            dyn_image: self.dyn_image.resize_exact(nw, nh, filter)
        }
    }

    /// Rotate this image by 90 degrees clockwise. Returns a new image
    pub fn rotate(&self, quarter: u8) -> Self {
        Self {
            dyn_image: match quarter % 4 {
                1 => self.dyn_image.rotate90(),
                2 => self.dyn_image.rotate180(),
                3 => self.dyn_image.rotate270(),
                _ => self.dyn_image.clone()
            }
        }
    }

    /// Flip this image horizontally or vertically. Returns a new image
    pub fn flip(&self, horizontal: bool) -> Self {
        Self {
            dyn_image: if horizontal {
                self.dyn_image.fliph()
            } else {
                self.dyn_image.flipv()
            }
        }
    }

    // Get a cut-out of this image delimited by the bounding rectangle. Returns a new image
    pub fn crop(&self, x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            dyn_image: self.dyn_image.crop_imm(x, y, w, h)
        }
    }

    /// Encode this image and get the encoded bytes. Returns a Vec<u8>
    pub fn buffer(&self, format: ImageOutputFormat) -> Result<Vec<u8>, ImageError> {
        let mut buf = Cursor::new(vec![]);
        match self.dyn_image.write_to(&mut buf, format) {
            Ok(_) => Ok(buf.into_inner()),
            Err(err) => Err(err)
        }
    }
}