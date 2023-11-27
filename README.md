# @fastigiata/image

> This is a node binding for [image](https://github.com/image-rs/image)

This package provides basic image processing functions and methods for converting to and from various image formats.

## Supported formats

see [supported-image-formats](https://github.com/image-rs/image#supported-image-formats) for more details

| Format          | From | To (untested) |
|-----------------|------|---------------|
| AVIF            |      |               |
| BMP             | ✓    | ✓             |
| DDS             |      |               |
| Farbfeld        | ✓    | ✓             |
| GIF             | ✓    | ✓             |
| ICO             | ✓    | ✓             |
| JPEG            | ✓    | ✓             |
| OpenEXR *       | ✓    | ✓             |
| PNG             | ✓    | ✓             |
| PNM             | ✓    | ✓             |
| QOI             | ✓    | ✓             |
| Radiance HDR ** | ✓    |               |
| TGA             | ✓    | ✓             |
| TIFF            | ✓    | ✓             |
| WebP            | ✓    |               |

`*`: Error: `The color Rgb32F can not be represented in PNG`  
`**`: Lossy, not present in the Rust library

## Improvements

- use `TypedArray` for image data instead of `Vec<u8>` to reduce unnecessary memory allocations