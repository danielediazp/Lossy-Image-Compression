use std::convert::TryInto;

/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let max_val: i128 = 1_i128 << (width - 1);
    let min_val: i128 = -(1_i128 << (width - 1));
    (n as i128) >= min_val && (n as i128) <= max_val
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    (n >> width) == 0
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    let word_length = 64;
    (((word << (word_length - width - lsb)) as i64) >> (word_length - width))
        .try_into()
        .unwrap()
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    let word_length = 64;
    (word << (word_length - width - lsb) as u64 >> (word_length - width))
        .try_into()
        .unwrap()
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if fitsu(value, width) {
        return Some((value << lsb) | word);
    }
    None
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitss(value, width) {
        let mask: i128 = (1_i128 << width) - 1;
        let val: u64 = (((value as i128) & (mask as i128)) << lsb) as u64;
        return Some(val | word);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::bitpack::*;

    #[test]
    fn test_fitsu() {
        // Test 9 bit
        assert_eq!(true, fitsu(511, 9));
        assert_eq!(false, fitsu(512, 9));
        // Test 5 bit
        assert_eq!(true, fitsu(31, 5));
        assert_eq!(false, fitsu(32, 5));
        // Test 4 bit
        assert_eq!(true, fitsu(15, 4));
        assert_eq!(false, fitsu(200, 4));
    }

    #[test]
    fn test_fitss(){
        // Test 9 bit
        assert_eq!(true, fitss(-256, 9));
        assert_eq!(true, fitss(254, 9));
        assert_eq!(false, fitss(-258, 9));
        assert_eq!(false, fitss(300, 9));
        // Test 5 bit
        assert_eq!(true, fitss(-16, 5));
        assert_eq!(true, fitss(15, 5));
        assert_eq!(false, fitss(-400, 5));
        assert_eq!(false, fitss(521, 5));
        // Test 4 bit
        assert_eq!(true, fitss(-8, 4));
        assert_eq!(true, fitss(7, 4));
        assert_eq!(false, fitss(-40, 4));
        assert_eq!(false, fitss(40, 4));
    }

    #[test]
    fn test_getu() {
        let mut word: u64 = 0_u64;
        let a: u64 = 511;
        let index_pb: u64 = 8;
        let index_pr: u64 = 10;
        word = newu(word, 9, 23, a).unwrap();
        word = newu(word, 4, 4, index_pb).unwrap();
        word = newu(word, 4, 0, index_pr).unwrap();
        let expected_a: u64 = getu(word, 9, 23);
        let extracted_index_pb: u64 = getu(word, 4, 4);
        let extracted_index_pr: u64 = getu(word, 4, 0);
        assert_eq!((a, index_pb, index_pr), (expected_a, extracted_index_pb, extracted_index_pr));
    }

    #[test]
    fn test_gets(){
        let mut word: u64 = 0_64;
        let b: i64 = -16;
        let c: i64 = -1;
        let d: i64 = -5;
        word = news(word, 5, 18, b).unwrap();
        word = news(word, 5, 13, c).unwrap();
        word = news(word, 5, 8, d).unwrap();
        let extracted_b: i64 = gets(word, 5, 18);
        let extracted_c: i64 = gets(word, 5, 13);
        let extracted_d: i64 = gets(word, 5, 8);
        assert_eq!((b, c, d), (extracted_b, extracted_c,  extracted_d));
    }
}