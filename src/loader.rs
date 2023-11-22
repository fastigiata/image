use image::{ImageFormat};
use napi::{Error, Result};
use crate::core::ImageWrapper;
use crate::common::CommonImage;

/// `ImageLoader` provides several way to load image binary into a `CommonImage`
#[napi]
pub struct ImageLoader {}

#[napi]
impl ImageLoader {
    #[inline]
    fn load(buffer: Vec<u8>, format: Option<ImageFormat>) -> Result<CommonImage> {
        match ImageWrapper::load(buffer, format) {
            Ok(iw) => Ok(CommonImage::new(iw)),
            Err(err) => Err(Error::from_reason(format!("{}", err))),
        }
    }

    /// Create a `CommonImage` instance from a byte slice. Makes an educated guess about the image format
    #[napi]
    pub fn auto_guess(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, None)
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a PNG
    #[napi]
    pub fn from_png(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Png))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a JPEG
    #[napi]
    pub fn from_jpeg(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Jpeg))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a GIF
    #[napi]
    pub fn from_gif(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Gif))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a WEBP
    #[napi]
    pub fn from_webp(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::WebP))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a PNM
    #[napi]
    pub fn from_pnm(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Pnm))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a TIFF
    #[napi]
    pub fn from_tiff(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Tiff))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a TGA
    #[napi]
    pub fn from_tga(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Tga))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a DDS
    #[napi]
    pub fn from_dds(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Dds))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a BMP
    #[napi]
    pub fn from_bmp(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Bmp))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a ICO
    #[napi]
    pub fn from_ico(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Ico))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a Radiance HDR
    #[napi]
    pub fn from_hdr(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Hdr))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a OpenEXR
    #[napi(js_name = "fromOpenEXR")]
    pub fn from_openexr(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::OpenExr))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a Farbfeld
    #[napi]
    pub fn from_farbfeld(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Farbfeld))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a AVIF
    #[napi]
    pub fn from_avif(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Avif))
    }

    /// Create a `CommonImage` instance from a byte slice. Assumes the image is a QOI
    #[napi]
    pub fn from_qoi(buffer: Vec<u8>) -> Result<CommonImage> {
        Self::load(buffer, Some(ImageFormat::Qoi))
    }
}