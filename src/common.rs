use image::codecs::pnm::{PnmSubtype, SampleEncoding};
use image::ImageOutputFormat;
use napi::{Error, Result};
use crate::core::ImageWrapper;

/// A wrapper around `ImageWrapper` that can be exposed to JavaScript
#[napi]
pub struct CommonImage {
    /// a wrapper around ImageBuffer that provides dynamic behavior
    wrapper: ImageWrapper,
}

#[napi]
impl CommonImage {
    /// Create a new CommonImage from a ImageWrapper without exposing the ImageWrapper to JavaScript
    pub fn new(wrapper: ImageWrapper) -> Self {
        CommonImage { wrapper }
    }


    #[inline]
    fn out(&self, format: ImageOutputFormat) -> Result<Vec<u8>> {
        self.wrapper.buffer(format).map_err(|err| Error::from_reason(format!("{}", err)))
    }

    /// Encode this image as a PNG and return the encoded bytes
    #[napi]
    pub fn to_png(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Png)
    }

    /// Encode this image as a JPEG(with specified quality) and return the encoded bytes
    ///
    /// ---
    /// `quality`: Valid within `1-100` and should be a `u8`, otherwise it will cause a panic
    #[napi]
    pub fn to_jpeg(&self, quality: u8) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Jpeg(quality))
    }

    /// Encode this image as a PNM(in variant PBM) and return the encoded bytes
    ///
    /// ---
    /// `binary_sample`: whether to use binary sample encoding, otherwise it will use ascii sample encoding. default is `true` for smaller size
    #[napi]
    pub fn to_pbm(&self, binary_sample: Option<bool>) -> Result<Vec<u8>> {
        let se = binary_sample.map_or(SampleEncoding::Binary, |b| {
            if b { SampleEncoding::Binary } else { SampleEncoding::Ascii }
        });
        self.out(ImageOutputFormat::Pnm(PnmSubtype::Bitmap(se)))
    }

    /// Encode this image as a PNM(in variant PGM) and return the encoded bytes
    ///
    /// ---
    /// `binary_sample`: whether to use binary sample encoding, otherwise it will use ascii sample encoding. default is `true` for smaller size
    #[napi]
    pub fn to_pgm(&self, binary_sample: Option<bool>) -> Result<Vec<u8>> {
        let se = binary_sample.map_or(SampleEncoding::Binary, |b| {
            if b { SampleEncoding::Binary } else { SampleEncoding::Ascii }
        });
        self.out(ImageOutputFormat::Pnm(PnmSubtype::Graymap(se)))
    }

    /// Encode this image as a PNM(in variant PPM) and return the encoded bytes
    ///
    /// ---
    /// `binary_sample`: whether to use binary sample encoding, otherwise it will use ascii sample encoding. default is `true` for smaller size
    #[napi]
    pub fn to_ppm(&self, binary_sample: Option<bool>) -> Result<Vec<u8>> {
        let se = binary_sample.map_or(SampleEncoding::Binary, |b| {
            if b { SampleEncoding::Binary } else { SampleEncoding::Ascii }
        });
        self.out(ImageOutputFormat::Pnm(PnmSubtype::Pixmap(se)))
    }

    /// Encode this image as a PNM(extended as PAM) and return the encoded bytes
    #[napi]
    pub fn to_pam(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Pnm(PnmSubtype::ArbitraryMap))
    }

    /// Encode this image as a PNM with specified subtype and sample encoding(if any), and return the encoded bytes
    ///
    /// ---
    /// see `to_pbm`, `to_pgm`, `to_ppm`, `to_pam` for more details
    pub fn to_pnm(&self, subtype: String, binary_sample: Option<bool>) -> Result<Vec<u8>> {
        match &*subtype {
            "pbm" => self.to_pbm(binary_sample),
            "pgm" => self.to_pgm(binary_sample),
            "ppm" => self.to_ppm(binary_sample),
            "pam" => self.to_pam(),
            _ => Err(Error::from_reason(format!("unknown pnm subtype: {}", subtype)))
        }
    }

    /// Encode this image as a GIF and return the encoded bytes
    #[napi]
    pub fn to_gif(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Gif)
    }

    /// Encode this image as a ICO and return the encoded bytes
    #[napi]
    pub fn to_ico(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Ico)
    }

    /// Encode this image as a BMP and return the encoded bytes
    #[napi]
    pub fn to_bmp(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Bmp)
    }

    /// Encode this image as a Farbfeld and return the encoded bytes
    #[napi]
    pub fn to_farbfeld(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Farbfeld)
    }

    /// Encode this image as a TGA and return the encoded bytes
    #[napi]
    pub fn to_tga(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Tga)
    }

    /// Encode this image as a OpenExr and return the encoded bytes
    #[napi(js_name = "toOpenExr")]
    pub fn to_openexr(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::OpenExr)
    }

    /// Encode this image as a TIFF and return the encoded bytes
    #[napi]
    pub fn to_tiff(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Tiff)
    }

    /// Encode this image as a QOI and return the encoded bytes
    #[napi]
    pub fn to_qoi(&self) -> Result<Vec<u8>> {
        self.out(ImageOutputFormat::Qoi)
    }
}