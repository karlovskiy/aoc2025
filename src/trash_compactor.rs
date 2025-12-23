pub fn part_one(data: &[u8]) -> u64 {
    let mut result = 0;
    let mut sums: Vec<u64> = vec![];
    let mut mults: Vec<u64> = vec![];
    let len = data.len();
    let mut k = 0;
    let mut num: u64 = 0;
    for i in 0..len {
        let b = unsafe { *data.get_unchecked(i) };
        if b.is_ascii_digit() {
            num = num * 10 + (b - b'0') as u64;
        } else if b == b' ' || b == b'\n' {
            if num == 0 {
                continue;
            }
            if k >= sums.len() {
                sums.push(num);
                mults.push(num);
            } else {
                unsafe {
                    *sums.get_unchecked_mut(k) += num;
                    *mults.get_unchecked_mut(k) *= num;
                }
            }
            num = 0;
            k = if b == b'\n' { 0 } else { k + 1 };
        } else if b == b'*' {
            result += unsafe { *mults.get_unchecked(k) };
            k += 1;
        } else if b == b'+' {
            result += unsafe { *sums.get_unchecked(k) };
            k += 1;
        }
    }
    result
}

pub fn part_two(data: &[u8]) -> u64 {
    let mut result = 0;
    let n = data.iter().position(|&b| b == b'\n').unwrap() + 1;
    let len = data.len();
    let mut is_sum = true;
    let mut col_sum: u64 = 0;
    let mut col_prod: u64 = 1;
    for i in 0..n {
        let mut step = i;
        let mut num = 0;
        while step < len {
            let b = unsafe { *data.get_unchecked(step) };
            if b.is_ascii_digit() {
                num = num * 10 + (b - b'0') as u64;
            } else if b == b'*' {
                is_sum = false;
            } else if b == b'+' {
                is_sum = true;
            }
            step += n;
        }
        if num != 0 {
            col_sum += num;
            col_prod *= num;
        }
        if num == 0 || i == n - 1 {
            if is_sum {
                result += col_sum;
            } else {
                result += col_prod;
            }
            col_sum = 0;
            col_prod = 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_bytes!("testdata/trash_compactor/example");
        let result = part_one(data);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part_one_input() {
        let data = include_bytes!("testdata/trash_compactor/input");
        let result = part_one(data);
        assert_eq!(result, 4405895212738);
    }

    #[test]
    fn part_two_example() {
        let data = include_bytes!("testdata/trash_compactor/example");
        let result = part_two(data);
        assert_eq!(result, 3263827);
    }

    #[test]
    fn part_two_input() {
        let data = include_bytes!("testdata/trash_compactor/input");
        let result = part_two(data);
        assert_eq!(result, 7450962489289);
    }
}
