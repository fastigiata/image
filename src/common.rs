use image::codecs::pnm::{PnmSubtype, SampleEncoding};
use image::imageops::FilterType;
use image::{ColorType, ImageOutputFormat};
use napi::{Error, Result};
use crate::core::ImageWrapper;

fn filter_parser(filter: &str) -> std::result::Result<FilterType, String> {
    match filter {
        "nearest" => Ok(FilterType::Nearest),
        "triangle" => Ok(FilterType::Triangle),
        "catmullRom" => Ok(FilterType::CatmullRom),
        "gaussian" => Ok(FilterType::Gaussian),
        "lanczos3" => Ok(FilterType::Lanczos3),
        _ => Err(format!("Invalid filter | {}", filter))
    }
}

fn strategy_parser(strategy: &str) -> std::result::Result<(&str, FilterType), String> {
    let parts: Vec<&str> = strategy.split('_').collect();

    if parts.len() != 2 {
        Err(format!("Invalid strategy | {}", strategy))
    } else {
        match parts[0] {
            "fit" | "cover" | "exact" => {
                match parts[1] {
                    "nearest" => Ok((parts[0], FilterType::Nearest)),
                    "triangle" => Ok((parts[0], FilterType::Triangle)),
                    "catmullRom" => Ok((parts[0], FilterType::CatmullRom)),
                    "gaussian" => Ok((parts[0], FilterType::Gaussian)),
                    "lanczos3" => Ok((parts[0], FilterType::Lanczos3)),
                    _ => return Err(format!("Invalid strategy | invalid filter: {}", parts[1]))
                }
            }
            _ => Err(format!("Invalid strategy | invalid mode: {}", parts[0]))
        }
    }
}

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

    // ========== ========== ========== ========== ==========
    // ========== ========== separator! ========== ==========
    // ========== ========== ========== ========== ==========
    #[napi(ts_return_type = "[width: number, height: number]")]
    pub fn dimensions(&self) -> Vec<u32> {
        let (w, h) = self.wrapper.dimensions();
        vec![w, h]
    }

    /// Get the color type of the image
    ///
    /// ---
    /// Return value is one of the following (there may be other values, use it with caution):
    /// - `l8`: 8-bit luminance
    /// - `la8`: 8-bit luminance with alpha channel
    /// - `rgb8`: 8-bit with R, G and B channels
    /// - `rgba8`: 8-bit with R, G, B and alpha channels
    /// - `l16`: 16-bit luminance
    /// - `la16`: 16-bit luminance with alpha channel
    /// - `rgb16`: 16-bit with R, G and B channels
    /// - `rgba16`: 16-bit with R, G, B and alpha channels
    /// - `rgb32f`: 32-bit floating point with R, G and B channels
    /// - `rgba32f`: 32-bit floating point with R, G, B and alpha channels
    /// - `unknown_since_non_exhaustive`: unknown color type, this should never happen
    #[napi(ts_return_type = "'l8'|'la8'|'rgb8'|'rgba8'|'l16'|'la16'|'rgb16'|'rgba16'|'rgb32f'|'rgba32f'|'unknown_since_non_exhaustive'")]
    pub fn color_type(&self) -> String {
        match self.wrapper.color() {
            ColorType::L8 => "l8",
            ColorType::La8 => "la8",
            ColorType::Rgb8 => "rgb8",
            ColorType::Rgba8 => "rgba8",
            ColorType::L16 => "l16",
            ColorType::La16 => "la16",
            ColorType::Rgb16 => "rgb16",
            ColorType::Rgba16 => "rgba16",
            ColorType::Rgb32F => "rgb32f",
            ColorType::Rgba32F => "rgba32f",
            _ => "unknown_since_non_exhaustive"
        }.to_string()
    }

    /// Bits per pixel (bpp) refers to the number of bits of information stored per pixel of the image
    #[napi]
    pub fn bpp(&self) -> u16 {
        self.wrapper.bits_per_pixel()
    }

    /// Resize this image using the specified filter algorithm. Returns a new image. The image's aspect ratio is preserved. The image is scaled to the maximum possible size that fits within the bounds specified by `nw` and `nh`.
    ///
    /// ---
    /// 'filter' can be one of the following (arranged from fastest to slowest):
    /// - `nearest`: Nearest Neighbor -- default
    /// - `triangle`: Linear Filter
    /// - `catmullRom`: Cubic Filter
    /// - `gaussian`: Gaussian Filter
    /// - `lanczos3`: Lanczos with window 3
    ///
    /// ---
    /// see {@link resizeToCover} and {@link resizeExact} for other resize strategies
    #[napi]
    pub fn resize_to_fit(
        &self, nw: u32, nh: u32,
        #[napi(ts_arg_type = "'nearest'|'triangle'|'catmullRom'|'gaussian'|'lanczos3'")]
        filter: Option<String>,
    ) -> Self {
        let filter = filter.map_or(FilterType::Nearest, |f| filter_parser(&f).unwrap());
        Self {
            wrapper: self.wrapper.resize_to_fit(nw, nh, filter)
        }
    }

    /// Resize this image using the specified filter algorithm. Returns a new image. The image's aspect ratio is preserved. The image is scaled to the maximum possible size that fits within the larger (relative to aspect ratio) of the bounds specified by `nw` and `nh`, then cropped to fit within the bounds specified by `nw` and `nh`.
    ///
    /// ---
    /// 'filter' can be one of the following (arranged from fastest to slowest):
    /// - `nearest`: Nearest Neighbor -- default
    /// - `triangle`: Linear Filter
    /// - `catmullRom`: Cubic Filter
    /// - `gaussian`: Gaussian Filter
    /// - `lanczos3`: Lanczos with window 3
    ///
    /// ---
    /// see {@link resizeToFit} and {@link resizeExact} for other resize strategies
    #[napi]
    pub fn resize_to_cover(
        &self, nw: u32, nh: u32,
        #[napi(ts_arg_type = "'nearest'|'triangle'|'catmullRom'|'gaussian'|'lanczos3'")]
        filter: Option<String>,
    ) -> Self {
        let filter = filter.map_or(FilterType::Nearest, |f| filter_parser(&f).unwrap());
        Self {
            wrapper: self.wrapper.resize_to_cover(nw, nh, filter)
        }
    }

    /// Resize this image using the specified filter algorithm. Returns a new image. Does not preserve aspect ratio. nw and nh are the new image's dimensions.
    ///
    /// ---
    /// 'filter' can be one of the following (arranged from fastest to slowest):
    /// - `nearest`: Nearest Neighbor -- default
    /// - `triangle`: Linear Filter
    /// - `catmullRom`: Cubic Filter
    /// - `gaussian`: Gaussian Filter
    /// - `lanczos3`: Lanczos with window 3
    ///
    /// ---
    /// see {@link resizeToFit} and {@link resizeToCover} for other resize strategies
    #[napi]
    pub fn resize_exact(
        &self, nw: u32, nh: u32,
        #[napi(ts_arg_type = "'nearest'|'triangle'|'catmullRom'|'gaussian'|'lanczos3'")]
        filter: Option<String>,
    ) -> Self {
        let filter = filter.map_or(FilterType::Nearest, |f| filter_parser(&f).unwrap());
        Self {
            wrapper: self.wrapper.resize_exact(nw, nh, filter)
        }
    }

    /// Rotate this image by 90 degrees clockwise. Returns a new image
    ///
    /// ---
    /// `quarter`: The number of 90-degree clockwise rotations to apply. Valid within `0-3` and should be a `u8`, otherwise it will cause a panic
    #[napi]
    pub fn rotate_quarter(&self, quarter: u8) -> Self {
        Self {
            wrapper: self.wrapper.rotate(quarter)
        }
    }

    /// Flip this image horizontally or vertically. Returns a new image
    ///
    /// ---
    /// `horizontal`: whether to flip horizontally, otherwise it will flip vertically. default is `true`
    #[napi]
    pub fn flip(&self, horizontal: Option<bool>) -> Self {
        Self {
            wrapper: self.wrapper.flip(horizontal.unwrap_or(true))
        }
    }

    /// Crop this image. Returns a new image
    #[napi]
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            wrapper: self.wrapper.crop(x, y, width, height)
        }
    }

    // ========== ========== ========== ========== ==========
    // ========== ========== separator! ========== ==========
    // ========== ========== ========== ========== ==========

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
    ///
    /// ---
    /// `strategy`: The strategy used when the width or height of the image exceeds 256. Its value is in the format of '&lt;mode&gt;_&lt;filter&gt;'.
    /// - `undefined`: No conversion is used, will cause a panic if the width or height of the image exceeds 256
    ///
    /// 'mode' can be one of the following:
    /// - `fit`: The image's aspect ratio is preserved. The image is scaled to the maximum possible size that fits within **256x256**.
    /// - `cover`: The image's aspect ratio is preserved. The image is scaled to the maximum possible size that fits within **256x256** (relative to aspect ratio), then cropped to fit within **256x256**.
    /// - `exact`: Does not preserve aspect ratio. **256x256** the new image's dimension.
    ///
    /// 'filter' can be one of the following (arranged from fastest to slowest):
    /// - `nearest`: Nearest Neighbor
    /// - `triangle`: Linear Filter
    /// - `catmullRom`: Cubic Filter
    /// - `gaussian`: Gaussian Filter
    /// - `lanczos3`: Lanczos with window 3
    #[napi]
    pub fn to_ico(
        &self,
        #[napi(ts_arg_type = "'fit_nearest'|'fit_triangle'|'fit_catmullRom'|'fit_gaussian'|'fit_lanczos3'|'cover_nearest'|'cover_triangle'|'cover_catmullRom'|'cover_gaussian'|'cover_lanczos3'|'exact_nearest'|'exact_triangle'|'exact_catmullRom'|'exact_gaussian'|'exact_lanczos3'")]
        strategy: Option<String>,
    ) -> Result<Vec<u8>> {
        let (w, h) = self.wrapper.dimensions();

        if w <= 256 && h <= 256 {
            self.out(ImageOutputFormat::Ico)
        } else {
            match strategy {
                Some(inner) => match strategy_parser(&inner) {
                    Ok((mode, filter)) => {
                        let transferred = match mode {
                            "fit" => self.wrapper.resize_to_fit(256, 256, filter),
                            "cover" => self.wrapper.resize_to_cover(256, 256, filter),
                            "exact" => self.wrapper.resize_exact(256, 256, filter),
                            _ => return Err(Error::from_reason(format!("This should never happen, please report this issue to me!"))),
                        };

                        // transferred.out(ImageOutputFormat::Ico)
                        transferred
                            .buffer(ImageOutputFormat::Ico)
                            .map_err(|err| Error::from_reason(format!("{}", err)))
                    }
                    Err(err_msg) => Err(Error::from_reason(err_msg))
                },
                None => Err(Error::from_reason(format!("image size is too large for ico format, max size is 256x256"))),
            }
        }
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

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let res = strategy_parser("fit_nearest");
        println!("{:?}", res);
    }
}