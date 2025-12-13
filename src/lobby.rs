
pub fn part_one(data: &[u8]) -> u64 {
    calc_total_joltage(data, 2)
}

pub fn part_two(data: &[u8]) -> u64 {
    calc_total_joltage(data, 12)
}

fn calc_total_joltage(data: &[u8], n: usize) -> u64 {
    let mut result: u64 = 0;
    let mut bank: Vec<u8> = vec![];
    for &c in data {
        if c == b'\n' {
            result += calc_bank_joltage(&mut bank, n);
            bank.clear();
            continue;
        }
        bank.push(c - 0x30); // ascii -> digit
    }
    result
}

fn calc_bank_joltage(bank: &mut Vec<u8>, n: usize) -> u64 {
    let mut result: u64 = 0;
    let mut pos = 0;
    for i in 0..n {
        let mut max = 0;
        for j in pos..=bank.len() - n + i {
            let x = bank[j];
            if x == 9 {
                pos = j;
                max = x;
                break;
            }
            if x > max {
                pos = j;
                max = x;
            }
        }
        if i == n - 1 {
            result += max as u64;
        } else {
            result += max as u64 * 10u64.pow((n - i - 1) as u32)
        }
        pos += 1;
    }
    debug_println!("{}: {:?}", result, bank);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_bytes!("testdata/lobby/example");
        let result = part_one(data);
        assert_eq!(result, 357);
    }

    #[test]
    fn part_one_input() {
        let data = include_bytes!("testdata/lobby/input");
        let total = part_one(data);
        assert_eq!(total, 17092);
    }

    #[test]
    fn part_two_example() {
        let data = include_bytes!("testdata/lobby/example");
        let result = part_two(data);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn part_two_input() {
        let data = include_bytes!("testdata/lobby/input");
        let total = part_two(data);
        assert_eq!(total, 170147128753455);
    }
}
