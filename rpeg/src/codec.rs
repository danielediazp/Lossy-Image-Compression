use crate::conversions;
use crate::conversions::{
    component_video_back_to_rbg_floats, fix_pixel_poss, from_blocks_to_component_format,
    from_dct_to_component_video, rgb_floats_to_rgb, unpack_values,
};
use array2::array2::Array2;
use conversions::blocks_to_dct;
use conversions::component_video_to_blocks;
use conversions::pack_values_into_word;
use conversions::rbg_floats_to_component_video;
use conversions::rgb_to_floats;
use csc411_image::{Read, Rgb, RgbImage, Write};
use csc411_rpegio::output_rpeg_data;
use csc411_rpegio::read_in_rpeg_data;

/// Takes a PPM image `filename` as input and reduces the size of the image by three times compared
/// to the original image. This is achieve through a lossy image compression process, which trades
/// pixel information for portability while keeping some pixel quality.
///
/// # Arguments
/// * `filename`: Location of the PPM within your disk
pub fn compress(filename: Option<&str>) {
    let original_image = RgbImage::read(filename.as_deref()).unwrap();
    let image_denominator = original_image.denominator;
    let image: Array2<Rgb> = Array2::from_even_dimension(
        original_image.width as usize,
        original_image.height as usize,
        original_image.pixels,
    );
    let rgb_floats_image = rgb_to_floats(image, image_denominator);
    let component_vide_form = rbg_floats_to_component_video(rgb_floats_image);
    let blocks_of_pixels = component_video_to_blocks(component_vide_form);
    let dct_coefficient = blocks_to_dct(blocks_of_pixels);
    let compressed_imag = pack_values_into_word(dct_coefficient);
    output_rpeg_data(
        &compressed_imag.data,
        compressed_imag.get_width() as u32,
        compressed_imag.get_height() as u32,
    );
}

pub fn decompress(filename: Option<&str>) {
    let compressed_image = read_in_rpeg_data(Some(filename.as_deref()).unwrap());
    let (byte_words, image_width, image_height) = compressed_image.unwrap();
    let image_data: Vec<u32> = byte_words
        .iter()
        .map(|val| u32::from_be_bytes(*val))
        .collect();
    let dct_arr = unpack_values(image_data, image_width as usize, image_height as usize);
    let blocks = from_dct_to_component_video(dct_arr);
    let cv_image = from_blocks_to_component_format(blocks);
    let rgb_float = component_video_back_to_rbg_floats(cv_image);
    let image = rgb_floats_to_rgb(rgb_float);
    let output = fix_pixel_poss(image.clone());
    let out_image: RgbImage = RgbImage {
        pixels: output,
        width: image.get_width() as u32,
        height: image.get_height() as u32,
        denominator: 255,
    };
    out_image.write(None).unwrap();
}
