use crate::structs::{Block, ComponentVideo, DCTCoefficient, RgbFloats};
use array2::array2::Array2;
use bitpack::bitpack::{gets, getu, news, newu};
use csc411_arith::{chroma_of_index, index_of_chroma};
use csc411_image::Rgb;

/// This functions takes an Rgb image stored in Array2 Struct with its denominator and converts
/// each pixel in the image into a floating point representation of the Rgb. It returns a new
/// Array2 of RgbFloats.
///
/// # Arguments
/// * `image` : Array2 of Rgb's representing the original image
pub fn rgb_to_floats(image: Array2<Rgb>, image_denominator: u16) -> Array2<RgbFloats> {
    let rgb_data: Vec<RgbFloats> = image
        .iter_row_major()
        .map(|pixel| compute_rgb_floats(pixel.2.clone(), image_denominator as f64))
        .collect();

    Array2::from_row_major(image.get_width(), image.get_height(), rgb_data)
}

/// This function takes and a Rgb pixel with the image denominator, and it turns the pixel
/// into a Floating point representation stored as an RgbFloats.
///=
/// # Arguments
/// * `pixel`: Rgb struct representing the red, green, and blue color density of the image
fn compute_rgb_floats(pixel: Rgb, denominator: f64) -> RgbFloats {
    let r = pixel.red as f64 / denominator;
    let g = pixel.green as f64 / denominator;
    let b = pixel.blue as f64 / denominator;

    RgbFloats {
        red: r,
        green: g,
        blue: b,
    }
}

/// This function takes an image where each Rgb value is represented as a floating
/// point, and it turns the floating point representation into a Component Video
/// representation of the pixels.
///
/// # Arguments
/// * `image`: Array2 of RgbFloats representing the image data
pub fn rbg_floats_to_component_video(image: Array2<RgbFloats>) -> Array2<ComponentVideo> {
    let cv: Vec<ComponentVideo> = image
        .iter_row_major()
        .map(|pixel| compute_component_video(pixel.2.clone()))
        .collect();

    Array2::from_row_major(image.get_width(), image.get_height(), cv)
}

/// This function takes the floating point representation of an Rgb and turns it
/// into a Component Video representation.
///
/// # Arguments
/// * `pixel`: Floating representation of the pixel red, green, and blue color density.
fn compute_component_video(pixel: RgbFloats) -> ComponentVideo {
    let y = 0.299 * pixel.red + 0.587 * pixel.green + 0.114 * pixel.blue;
    let pb = -0.168736 * pixel.red - 0.331264 * pixel.green + 0.5 * pixel.blue;
    let pr = 0.5 * pixel.red - 0.418688 * pixel.green - 0.081312 * pixel.blue;

    ComponentVideo { y, pb, pr }
}

/// This function takes a Array2 of ComponentVideo struct which represent an image
/// in component video format, and it extracts the 2x2 block of pixels to further
/// undergo under compression.
///
/// # Arguments
/// * `image_in_component_vid`: Array2 where each pixel is represent in Component Video format
pub fn component_video_to_blocks(image_in_component_vid: Array2<ComponentVideo>) -> Array2<Block> {
    let width = image_in_component_vid.get_width();
    let height = image_in_component_vid.get_height();
    let mut block_arr: Vec<Block> = Vec::new();
    for row in (0..height).step_by(2) {
        for col in (0..width).step_by(2) {
            let block = get_block(& image_in_component_vid, row, col);
            block_arr.push(block);
        }
    }

    Array2::from_row_major(width, height, block_arr)
}

/// This functions gets the 2x2 Block at the current `row` and `col`. The current
/// `row` `col` location can be consider as the top right, `row` `col` + 1 is the
/// top left, `row` + 1 `col` is the bottom right , and `row` + 1 `col` + 1 is the bottom
/// left pixel in the Block. This function servers as a helper function to component_video_to_blocks
///
/// # Arguments
/// * `cv_arr` : A 1D array representing a 2D matrix of ComponentVideo pixels
/// * `row` : Current row that you are trying to get the block for "it must be step by 2"
/// * `col`: Current col that you are trying to get the block for "it must be step by 2"
fn get_block(cv_arr: &Array2<ComponentVideo>, row: usize, col: usize) -> Block {
    let y1 = cv_arr.get(col, row).unwrap().clone();
    let y2 = cv_arr.get(col + 1, row).unwrap().clone();
    let y3 = cv_arr.get(col, row + 1).unwrap().clone();
    let y4 = cv_arr.get(col + 1, row + 1).unwrap().clone();

    Block { y1, y2, y3, y4 }
}

/// Takes a 2x2 block of pixels represented in ComponentVideo format, and turn this block
/// into DCTCoefficient values.
///
/// # Arguments
/// `blocks`: block of 2x2 pixels of ComponentVideo format
pub fn blocks_to_dct(blocks: Array2<Block>) -> Array2<DCTCoefficient> {
    let dct_arr: Vec<DCTCoefficient> = blocks
        .data
        .iter()
        .map(|block| compute_dct((*block).clone()))
        .collect();

    Array2::from_row_major(blocks.get_width(), blocks.get_height(), dct_arr)
}

/// This function compute the DCTCoefficient of a 2x2 Block of ComponentVideos. It serves
/// as a helper function for block_to_dct.
///
/// # Arguments
/// `block`: 2x2 block of ComponentVideo
fn compute_dct(block: Block) -> DCTCoefficient {
    let denominator: f64 = 4.0;
    let y1 = block.y1;
    let y2 = block.y2;
    let y3 = block.y3;
    let y4 = block.y4;
    let a = (y4.y + y3.y + y2.y + y1.y) / denominator;
    let b = (y4.y + y3.y - y2.y - y1.y) / denominator;
    let c = (y4.y - y3.y + y2.y - y1.y) / denominator;
    let d = (y4.y - y3.y - y2.y + y1.y) / denominator;
    let average_pb = (y1.pb + y2.pb + y3.pb + y4.pb) / denominator;
    let average_pr = (y1.pr + y2.pr + y3.pr + y4.pr) / denominator;

    DCTCoefficient {
        a: (a * 511.0).round(),
        b: (b.clamp(-0.3, 0.3) * 50.0).round(),
        c: (c.clamp(-0.3, 0.3) * 50.0).round(),
        d: (d.clamp(-0.3, 0.3) * 50.0).round(),
        index_of_pb: index_of_chroma(average_pb as f32),
        index_of_pr: index_of_chroma(average_pr as f32),
    }
}

/// This function takes Array2 Struct of the DCTCoefficient that are obtained from each
/// 2x2 block of pixel inside the original image, and it pack each DCTCoefficient word
/// into a 32 bit word that is been represented as a 64 bit word for the purpose of not
/// over-floating the bit word.
///
/// # Arguments
/// * `dct_arr`: Array2 Struct of dct coefficient values calculated from the 2x2 blocks of pixels
pub fn pack_values_into_word(dct_arr: Array2<DCTCoefficient>) -> Array2<[u8; 4]> {
    let mut compressed_image = Vec::new();
    for dct_coefficient in dct_arr.data.iter() {
        let mut word: u64 = 0_64;
        word = newu(word, 9, 23, dct_coefficient.a as u64).unwrap();
        word = news(word, 5, 18, dct_coefficient.b as i64).unwrap();
        word = news(word, 5, 13, dct_coefficient.c as i64).unwrap();
        word = news(word, 5, 8, dct_coefficient.d as i64).unwrap();
        word = newu(word, 4, 4, dct_coefficient.index_of_pb as u64).unwrap();
        word = newu(word, 4, 0, dct_coefficient.index_of_pr as u64).unwrap();
        compressed_image.push((word as u32).to_be_bytes());
    }

    Array2::from_row_major(dct_arr.get_width(), dct_arr.get_height(), compressed_image)
}

// Decompression

/// Takes a binary representation of pack DTCCoefficient values into 32 bit words, and it converges
/// the values back to DCTCoefficients. Returns an Array2 Struct of DTCCoefficients.
///
/// # Arguments:
/// `compressed_imag`: A compressed image into 32 bits code words.
pub fn unpack_values(
    compressed_imag: Vec<u32>,
    image_width: usize,
    image_height: usize,
) -> Array2<DCTCoefficient> {
    let mut dct_arr: Vec<DCTCoefficient> = Vec::new();
    for image_data in compressed_imag.iter() {
        let word = image_data.clone() as u64;
        let a = getu(word, 9, 23);
        let b = gets(word, 5, 18);
        let c = gets(word, 5, 13);
        let d = gets(word, 5, 8);
        let pb = getu(word, 4, 4);
        let pr = getu(word, 4, 0);

        dct_arr.push(DCTCoefficient {
            a: a as f64,
            b: b as f64,
            c: c as f64,
            d: d as f64,
            index_of_pb: pb as usize,
            index_of_pr: pr as usize,
        });
    }

    Array2::from_row_major(image_width, image_height, dct_arr)
}

/// This functions takes an Array2 Struct of DCTCoefficients and convert each coefficient back
/// to a 2x2 block of ComponentVideo pixels. Returns an Array2 with each separated Block.
///
/// # Arguments:
/// * `dct_arr`: Array2 of DCTCoefficient representing block of 2x2 pixels
pub fn from_dct_to_component_video(dct_arr: Array2<DCTCoefficient>) -> Array2<Block> {
    let block: Vec<Block> = dct_arr
        .data
        .iter()
        .map(|coefficient| from_dct_to_block((*coefficient).clone()))
        .collect();
    Array2::from_row_major(dct_arr.get_width(), dct_arr.get_height(), block)
}

/// Takes a DCTCoefficient and converges the coefficient to a 2x2 block of component video;
///
/// # Argument
/// * `coefficient`: DCTCoefficient storing the information of the 2x2 block of pixels
fn from_dct_to_block(coefficient: DCTCoefficient) -> Block {
    // Quantized representation of DCTCoefficient
    let a = (coefficient.a / 511.0).clamp(0.0, 1.0);
    let b = (coefficient.b / 50.0).clamp(-0.3, 0.3);
    let c = (coefficient.c / 50.0).clamp(-0.3, 0.3);
    let d = (coefficient.d / 50.0).clamp(-0.3, 0.3);
    // Compute the y value for each block
    let y1 = a - b - c + d;
    let y2 = a - b + c - d;
    let y3 = a + b - c - d;
    let y4 = a + b + c + d;
    // Get the lumin of each block
    let pb = chroma_of_index(coefficient.index_of_pb) as f64;
    let pr = chroma_of_index(coefficient.index_of_pr) as f64;
    // get block
    let top_left = dct_to_component_video(y1, pb, pr);
    let top_right = dct_to_component_video(y2, pb, pr);
    let bottom_left = dct_to_component_video(y3, pb, pr);
    let bottom_right = dct_to_component_video(y4, pb, pr);

    Block {
        y1: top_left,
        y2: top_right,
        y3: bottom_left,
        y4: bottom_right,
    }
}

/// Takes a y, pb, pr and returns a ComponentVideo Struct storing the values;
///
/// # Arguments
/// * `y`: Luminance value of the pixel
/// * `pb`: First side channel that transmit color difference signals
/// * `pr`: Second side channel that transmit color difference signals
fn dct_to_component_video(y: f64, index_of_pb: f64, index_of_pr: f64) -> ComponentVideo {
    ComponentVideo {
        y,
        pb: index_of_pb,
        pr: index_of_pr,
    }
}

/// This function takes an Array2 Struct 2x2 block of ComponentVideo representing pixel in this format
/// , and it return an Array2 struct of ComponentVideo where each pixel is located at its core spot.
/// Returns a Array2 Struct of ComponentVideo.
///
/// # Argument
/// * `block`: Block of 2x2 pixel represented in ComponentVideo format
pub fn from_blocks_to_component_format(block: Array2<Block>) -> Array2<ComponentVideo> {
    let mut cv_image: Vec<ComponentVideo> = Vec::new();
    for pixel_block in block.data.iter() {
        cv_image.push(pixel_block.y1.clone());
        cv_image.push(pixel_block.y2.clone());
        cv_image.push(pixel_block.y3.clone());
        cv_image.push(pixel_block.y4.clone());
    }
    Array2::from_row_major(block.get_width(), block.get_height(), cv_image)
}

/// This function takes an Array2 of pixels in ComponentVideo format and translate each pixel
/// back into its RgbFloat representation. Return Array2 Struct of RgbFloats.
///
/// # Argument
/// * `cv_image`: Image of pixels in ComponentVideo format
pub fn component_video_back_to_rbg_floats(cv_image: Array2<ComponentVideo>) -> Array2<RgbFloats> {
    let rgb_float_arr: Vec<RgbFloats> = cv_image
        .iter_row_major()
        .map(|cv| component_back_to_rgb_floats(cv.2.clone()))
        .collect();

    Array2::from_row_major(cv_image.get_width(), cv_image.get_height(), rgb_float_arr)
}

/// This function takes a pixel represented in ComponentVideo format, and it converges the pixel
/// back to RgbFloat format. Returns a RgbFloat struct with the pixel data.
///
/// # Argument
/// * `cv`: pixel in ComponentVideo format
fn component_back_to_rgb_floats(cv: ComponentVideo) -> RgbFloats {
    let red = (1.0 * cv.y + 0.0 * cv.pb + 1.402 * cv.pr) * 255.0;
    let green = (1.0 * cv.y - 0.344136 * cv.pb - 0.714136 * cv.pr) * 255.0;
    let blue = (1.0 * cv.y + 1.772 * cv.pb + 0.0 * cv.pr) * 255.0;

    RgbFloats { red, green, blue }
}

/// This function takes an Array2 Struct of pixels represented as RgbFloats and normalizes
/// each pixel back to Rgb format. Returns a new Array2 struct of Rgb which can be consider
/// an full decompressed image.
///
/// # Arguments
/// * `rgb_float_arr`: Array2 of Rgb's represented as floating point values
pub fn rgb_floats_to_rgb(rgb_float_arr: Array2<RgbFloats>) -> Array2<Rgb> {
    let image: Vec<Rgb> = rgb_float_arr
        .iter_row_major()
        .map(|pixel| from_rgb_float_to_rgb(pixel.2.clone()))
        .collect();

    Array2::from_row_major(rgb_float_arr.get_width(), rgb_float_arr.get_height(), image)
}

/// This function takes an pixel represented as floating point value and converges the pixel back
/// to a normal Rgb pixel. Returns a pixel on Rgb format.
///
/// # Argument
/// * `pixel`: pixel which red, green, and blue density are represented as floating point
fn from_rgb_float_to_rgb(pixel: RgbFloats) -> Rgb {
    let red = pixel.red as u16;
    let green = pixel.green as u16;
    let blue = pixel.blue as u16;

    Rgb { red, green, blue }
}

pub fn fix_pixel_poss(image: Array2<Rgb>) -> Vec<Rgb> {
    let width = image.get_width();
    let height = image.get_height();
    let mut output: Vec<Rgb> = vec![
        Rgb {
            red: 0,
            blue: 0,
            green: 0
        };
        width * height
    ];
    let mut counter = 0;
    for r in (0..height).step_by(2) {
        for c in (0..width).step_by(2) {
            let mut idx = r * width + c;
            output[idx] = image.data[counter].clone();
            counter += 1;
            idx = r * width + (c + 1);
            output[idx] = image.data[counter].clone();
            counter += 1;
            idx = (r + 1) * width + c;
            output[idx] = image.data[counter].clone();
            counter += 1;
            idx = (r + 1) * width + (c + 1);
            output[idx] = image.data[counter].clone();
            counter += 1;
        }
    }
    output
}
