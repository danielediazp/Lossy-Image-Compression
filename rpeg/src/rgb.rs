use crate::structs::{ComponentVideo, RgbFloats};
use csc411_image::Rgb;

/// This function takes and a Rgb pixel with the image denominator, and it turns the pixel
/// into a Floating point representation stored as an RgbFloats.
///=
/// # Arguments
/// * `pixel`: Rgb struct representing the red, green, and blue color density of the image
pub fn compute_rgb_floats(pixel: Rgb, denominator: f64) -> RgbFloats {
    let r = pixel.red as f64 / denominator;
    let g = pixel.green as f64 / denominator;
    let b = pixel.blue as f64 / denominator;

    RgbFloats {
        red: r,
        green: g,
        blue: b,
    }
}

// Decompression

/// This function takes an pixel represented as floating point value and converges the pixel back
/// to a normal Rgb pixel. Returns a pixel on Rgb format.
///
/// # Argument
/// * `pixel`: pixel which red, green, and blue density are represented as floating point
pub fn from_rgb_float_to_rgb(pixel: RgbFloats) -> Rgb {
    let red = pixel.red as u16;
    let green = pixel.green as u16;
    let blue = pixel.blue as u16;

    Rgb { red, green, blue }
}

/// This function takes a pixel represented in ComponentVideo format, and it converges the pixel
/// back to RgbFloat format. Returns a RgbFloat struct with the pixel data.
///
/// # Argument
/// * `cv`: pixel in ComponentVideo format
pub fn component_back_to_rgb_floats(cv: ComponentVideo) -> RgbFloats {
    let red = (1.0 * cv.y + 0.0 * cv.pb + 1.402 * cv.pr) * 255.0;
    let green = (1.0 * cv.y - 0.344136 * cv.pb - 0.714136 * cv.pr) * 255.0;
    let blue = (1.0 * cv.y + 1.772 * cv.pb + 0.0 * cv.pr) * 255.0;

    RgbFloats { red, green, blue }
}
