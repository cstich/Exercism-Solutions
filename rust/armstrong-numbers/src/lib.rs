use std::convert::TryFrom;
const RADIX: u32 = 10;

pub fn is_armstrong_number(num: u32) -> bool {
    let y = num.to_string();
    let n_chars: u32 = u32::try_from(y.len()).unwrap();

    let sum = y
        .chars()
        .map(|c| u32::pow(c.to_digit(RADIX).unwrap(), n_chars))
        .sum::<u32>();

    sum == num
}
