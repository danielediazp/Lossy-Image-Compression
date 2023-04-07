#[derive(Clone, Debug)]
/// ## Represent a Rgb value as a floating point value.
///
/// This struct is used to represent an Rgb value as a floating point value
/// within the arith compression and decompression implementation. In this struct,
/// the red, green, and blue field all f64s.
///
/// # Usage Example
///
/// ```
/// use rpeg::structs::RgbFloats;
///
/// let rgb_float = RgbFloats{ red: 0.0, green: 16.6, blue: 25.5};
///
/// ```
///
pub struct RgbFloats {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Clone, Debug)]
/// ## Represent the Component video format
///
/// This struct is used within arith in the compression and decompression. In the compression
/// step, the RgbFloats are converted into ComponentVideo format. In the decompression step,
/// the ComponentVideo Struct are converted back to RgbFloats.
///
/// # Usage Example
///
/// ```
/// use rpeg::structs::ComponentVideo;
///
/// let cv_format = ComponentVideo{y: 0.16, pb: 0.14, pr: 0.16};
/// ```
pub struct ComponentVideo {
    pub y: f64,
    pub pb: f64,
    pub pr: f64,
}

#[derive(Clone, Debug)]
/// # Represent the 2x2 block of pixels converted into ComponentVideo format
///
/// This struct is used within arith in the compression and decompression. In the compression
/// step, 2x2 block of pixel translated into ComponentVideo are stored as single struct . In the
/// decompression step, the Block is translated back into 4 different ComponentVideos.
///
/// # Usage Example
/// ```
/// use rpeg::structs::{ComponentVideo, Block};
///
/// let cv_1 = ComponentVideo{y: 0.16, pb: 0.14, pr: 0.16};
/// let cv_2 = ComponentVideo{y: 0.22, pb: 0.22, pr: 0.};
/// let cv_3 = ComponentVideo{y: 0.16, pb: 0.14, pr: 0.16};
/// let cv_4 = ComponentVideo{y: 0.16, pb: 0.14, pr: 0.16};
/// let dst_format = Block{y1: cv_1, y2: cv_2, y3: cv_3, y4: cv_4};
/// ```
pub struct Block {
    pub y1: ComponentVideo,
    pub y2: ComponentVideo,
    pub y3: ComponentVideo,
    pub y4: ComponentVideo,
}

#[derive(Clone, Debug)]
/// # Represent a discrete cosine transformation of the 4 pixels (luminance/luma)
///
/// This struct stores the discrete cosine transformation apply to the 2x2 pixel blocks
/// in the image that will be further pack into a 32-bit words. It can be found within
/// the compression and decompression stage of arith.
///
/// # Usage Example
///
/// ```
/// use rpeg::structs::DCTCoefficient;
///
/// let coefficient = DCTCoefficient{a: 0.1, b: -0.13, c: 0.155, d: 0.3335, index_of_pb: 4, index_of_pr: 5 };
/// ```
pub struct DCTCoefficient {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub index_of_pb: usize,
    pub index_of_pr: usize,
}
