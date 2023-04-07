use crate::structs::{Block, ComponentVideo, RgbFloats};
use array2::array2::Array2;

/// This function takes the floating point representation of an Rgb and turns it
/// into a Component Video representation.
///
/// # Arguments
/// * `pixel`: Floating representation of the pixel red, green, and blue color density.
pub fn compute_component_video(pixel: RgbFloats) -> ComponentVideo {
    let y = 0.299 * pixel.red + 0.587 * pixel.green + 0.114 * pixel.blue;
    let pb = -0.168736 * pixel.red - 0.331264 * pixel.green + 0.5 * pixel.blue;
    let pr = 0.5 * pixel.red - 0.418688 * pixel.green - 0.081312 * pixel.blue;

    ComponentVideo { y, pb, pr }
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
pub fn get_block(cv_arr: &Array2<ComponentVideo>, row: usize, col: usize) -> Block {
    let y1 = cv_arr.get(col, row).unwrap().clone();
    let y2 = cv_arr.get(col + 1, row).unwrap().clone();
    let y3 = cv_arr.get(col, row + 1).unwrap().clone();
    let y4 = cv_arr.get(col + 1, row + 1).unwrap().clone();

    Block { y1, y2, y3, y4 }
}
