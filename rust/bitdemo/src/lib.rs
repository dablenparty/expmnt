/// https://en.wikipedia.org/wiki/Fast_inverse_square_root
pub fn rsqrt(number: f32) -> f32 {
    const THREE_HALVES: f32 = 1.5;
    let transmuted = number.to_bits();
    let bit_hack = 0x5f3759df - (transmuted >> 1);
    let y = f32::from_bits(bit_hack);
    // this can be done a second time, but it's not necessary
    let x2 = number * 0.5;
    y * (THREE_HALVES - (x2 * y * y))
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use super::*;

    #[test]
    fn test_rsqrt() {
        let expected = 0.5f32;
        // this funky line rounds to 1 decimal place
        // the function is an approximation anyway
        let actual = rsqrt(4.0f32).mul(10.0).round().mul(0.1);
        assert_eq!(expected, actual);
    }
}
