use crate::structs::{Block, ComponentVideo, DCTCoefficient};
use csc411_arith::{chroma_of_index, index_of_chroma};

/// This function compute the DCTCoefficient of a 2x2 Block of ComponentVideos. It serves
/// as a helper function for block_to_dct.
///
/// # Arguments
/// `block`: 2x2 block of ComponentVideo
pub fn compute_dct(block: Block) -> DCTCoefficient {
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
    // Quantized DCTCoefficients values
    DCTCoefficient {
        a: (a * 511.0).round(),
        b: (b.clamp(-0.3, 0.3) * 50.0).round(),
        c: (c.clamp(-0.3, 0.3) * 50.0).round(),
        d: (d.clamp(-0.3, 0.3) * 50.0).round(),
        index_of_pb: index_of_chroma(average_pb as f32),
        index_of_pr: index_of_chroma(average_pr as f32),
    }
}

/// Takes a DCTCoefficient and converges the coefficient to a 2x2 block of component video;
///
/// # Argument
/// * `coefficient`: DCTCoefficient storing the information of the 2x2 block of pixels
pub fn from_dct_to_block(coefficient: DCTCoefficient) -> Block {
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
