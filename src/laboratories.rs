pub fn part_one(data: &[u8]) -> u64 {
    let (start, step) = get_start_and_step(data);
    let mut starts = Vec::with_capacity(100);
    starts.push(start);
    let len = data.len();
    let mut processed = vec![false; len];
    let mut result = 0;
    while !starts.is_empty() {
        let mut pos = starts.pop().unwrap();
        loop {
            pos += step;
            if pos >= len {
                break;
            } else if unsafe { *data.get_unchecked(pos) } == b'^' {
                if unsafe { !processed.get_unchecked(pos) } {
                    let right = pos + 1;
                    if right < len {
                        starts.push(right);
                    }
                    starts.push(pos - 1);
                    unsafe {
                        *processed.get_unchecked_mut(pos) = true;
                    }
                    result += 1;
                }
                break;
            }
        }
    }
    result
}

pub fn part_two(data: &[u8]) -> u64 {
    let (start, step) = get_start_and_step(data);
    let mut line = vec![0; step];
    line[start] = 1;
    let mut row_start = step * 2;
    while row_start < data.len() {
        for i in 0..step {
            let count = unsafe { *line.get_unchecked(i) };
            if count > 0 {
                if unsafe { *data.get_unchecked(row_start + i) } == b'^' {
                    line[i - 1] += count;
                    line[i + 1] += count;
                    line[i] = 0;
                }
            }
        }
        row_start += step * 2; // skip 2th rows
    }
    line.iter().sum()
}

pub fn part_two_recursion(data: &[u8]) -> u64 {
    let (start, step) = get_start_and_step(data);
    let mut cache = vec![u64::MAX; data.len()];
    search(start, step, data, &mut cache)
}

fn search(start: usize, step: usize, data: &[u8], cache: &mut Vec<u64>) -> u64 {
    let count = unsafe { *cache.get_unchecked(start) };
    if count != u64::MAX {
        return count;
    }
    let mut pos = start;
    let len = data.len();
    loop {
        pos += step;
        if pos >= len {
            return 1;
        } else if unsafe { *data.get_unchecked(pos) } == b'^' {
            let mut count = 0;
            let right = pos + 1;
            if right < len {
                count += search(right, step, data, cache)
            }
            let left = pos - 1;
            count += search(left, step, data, cache);
            unsafe {
                *cache.get_unchecked_mut(start) = count;
            }
            return count;
        }
    }
}

fn get_start_and_step(data: &[u8]) -> (usize, usize) {
    let mut start = 0;
    let mut step = 0;
    for (i, &b) in data.iter().enumerate() {
        if b == b'S' {
            start = i;
        } else if b == b'\n' {
            step = i + 1;
            break;
        }
    }
    (start, step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_example() {
        let data = include_bytes!("testdata/laboratories/example");
        let result = part_two(data);
        assert_eq!(result, 40);
    }

    #[test]
    fn part_two_input() {
        let data = include_bytes!("testdata/laboratories/input");
        let result = part_two(data);
        assert_eq!(result, 15811946526915);
    }

    #[test]
    fn part_two_recursion_example() {
        let data = include_bytes!("testdata/laboratories/example");
        let result = part_two_recursion(data);
        assert_eq!(result, 40);
    }

    #[test]
    fn part_two_recursion_input() {
        let data = include_bytes!("testdata/laboratories/input");
        let result = part_two_recursion(data);
        assert_eq!(result, 15811946526915);
    }
}
