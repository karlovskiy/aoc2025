use crate::debug_println;
use std::collections::HashSet;
pub fn part_one(data: &[u8]) -> u64 {
    calc_invalid_ids(data, false)
}

pub fn part_two(data: &[u8]) -> u64 {
    calc_invalid_ids(data, true)
}

fn calc_invalid_ids(data: &[u8], new_rules: bool) -> u64 {
    let mut invalid_ids = HashSet::new();
    let [mut a, mut b, mut x, mut prev_idx] = [0; 4];
    let mut prev = 0;
    for (i, &c) in data.iter().enumerate() {
        if c == b'-' {
            b = prev_idx;
        } else if prev == b'-' {
            x = i;
        } else if c == b',' || c == b'\n' {
            let start = str::from_utf8(&data[a..b + 1])
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let end = str::from_utf8(&data[x..prev_idx + 1])
                .unwrap()
                .parse::<u64>()
                .unwrap();
            if new_rules {
                calc_range_part_two(&mut invalid_ids, start, end)
            } else {
                calc_range_part_one(&mut invalid_ids, start, end);
            }
        } else if prev == b',' {
            a = i;
        }
        prev_idx = i;
        prev = c;
    }
    let result: u64 = invalid_ids.iter().sum();
    result
}

fn calc_range_part_one(invalid_ids: &mut HashSet<u64>, start: u64, end: u64) {
    debug_println!("{} - {}", start, end);
    let mut digits = [0u8; 20];
    let start_len = to_digits(start, &mut digits);
    let end_len = to_digits(end, &mut digits);
    if start_len % 2 != 0 && start_len == end_len {
        return;
    }
    for x in start..=end {
        let len = to_digits(x, &mut digits);
        if len % 2 != 0 {
            continue;
        }
        let mid = len / 2;
        let mut invalid = true;
        for i in 0..mid {
            let a = digits[i];
            let b = digits[i + mid];
            if a != b {
                invalid = false;
                break;
            }
        }
        if invalid {
            invalid_ids.insert(x);
            debug_println!("  invalid: {}", x);
        }
    }
}

fn calc_range_part_two(invalid_ids: &mut HashSet<u64>, start: u64, end: u64) {
    debug_println!("{} - {}", start, end);
    let mut digits = [0u8; 20];
    for x in start..=end {
        let len = to_digits(x, &mut digits);
        let mid = len / 2;
        let mut invalid = false;
        for n in 1..=mid {
            if len % n != 0 {
                continue;
            }
            let mut found = true;
            for i in 0..(len - n) {
                if digits[i] != digits[i + n] {
                    found = false;
                    break;
                }
            }
            if found {
                invalid = true;
                break;
            }
        }
        if invalid {
            invalid_ids.insert(x);
            debug_println!("  invalid: {}", x);
        }
    }
}

fn to_digits(mut x: u64, digits: &mut [u8]) -> usize {
    let mut len = 0;
    if x == 0 {
        digits[0] = 0;
        len = 1;
    } else {
        while x > 0 {
            digits[len] = (x % 10) as u8;
            x /= 10;
            len += 1;
        }
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_bytes!("testdata/gift_shop/example");
        let result = part_one(data);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part_one_input() {
        let data = include_bytes!("testdata/gift_shop/input");
        let total = part_one(data);
        assert_eq!(total, 12586854255);
    }

    #[test]
    fn part_two_example() {
        let data = include_bytes!("testdata/gift_shop/example");
        let result = part_two(data);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn part_two_input() {
        let data = include_bytes!("testdata/gift_shop/input");
        let total = part_two(data);
        assert_eq!(total, 17298174201);
    }
}
