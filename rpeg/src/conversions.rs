use crate::structs::{Block, ComponentVideo, DCTCoefficient, RgbFloats};
use array2::array2::Array2;
use bitpack::bitpack::{gets, getu, news, newu};
//use csc411_arith::{chroma_of_index, index_of_chroma};
use crate::component_video_and_blocks::{compute_component_video, get_block};
use crate::dct_coeff::{compute_dct, from_dct_to_block};
use crate::rgb::{component_back_to_rgb_floats, compute_rgb_floats, from_rgb_float_to_rgb};
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
            let block = get_block(&image_in_component_vid, row, col);
            block_arr.push(block);
        }
    }

    Array2::from_row_major(width, height, block_arr)
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

/// This function takes Array2 Struct of the DCTCoefficient that are obtained from each
/// 2x2 block of pixel inside the original image, and it pack each DCTCoefficient word
/// into a 32 bit word that is been represented as a 64 bit word for the purpose of not
/// over-floating the bit word.
///
/// # Arguments
/// * `dct_arr`: Array2 Struct of dct coefficient values calculated from the 2x2 blocks of pixels
pub fn pack_values_into_word(dct_arr: Array2<DCTCoefficient>) -> Array2<[u8; 4]> {
    let mut output_image = Vec::new();
    for value in dct_arr.data.iter() {
        let mut word = 0_u64;
        word = newu(word, 9, 23, value.a as u64).unwrap();
        word = news(word, 5, 18, value.b as i64).unwrap();
        word = news(word, 5, 13, value.c as i64).unwrap();
        word = news(word, 5, 8, value.d as i64).unwrap();
        word = newu(word, 4, 4, value.index_of_pb as u64).unwrap();
        word = newu(word, 4, 0, value.index_of_pr as u64).unwrap();
        output_image.push((word as u32).to_be_bytes());
    }

    Array2::from_row_major(dct_arr.get_width(), dct_arr.get_height(), output_image)
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

/// This function takes a decompressed image inside Array2 Struct of Rgb's and fix the pixels
/// at their expected position. Returns a fully decompressed image.
///
/// When decompressing the 2x2 Block of pixels, the blocks are stored in row-major order. The
/// main purpose of this function is to return the blocks back to block 2x2 order.
///
/// # Arguments
/// * `image`: Array2 Struct of Rgb where blocks are stored in row-major order.
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
