pub fn part_one(data: &[u8]) -> u64 {
    let mut result = 0;
    let stride = data.iter().position(|&b| b == b'\n').unwrap() as isize + 1;
    let len = data.len() as isize;
    for (i, &b) in data.iter().enumerate() {
        if b == b'@' {
            let i = i as isize;
            let mut count: u8 = 0;
            let l = i - 1;
            if l >= 0 && unsafe { *data.get_unchecked(l as usize) } == b'@' {
                count += 1;
            }
            let r = i + 1;
            if r < len && unsafe { *data.get_unchecked(r as usize) } == b'@' {
                count += 1;
            }
            let ul = i - stride - 1;
            if ul >= 0 && unsafe { *data.get_unchecked(ul as usize) } == b'@' {
                count += 1;
            }
            let u = i - stride;
            if u >= 0 && unsafe { *data.get_unchecked(u as usize) } == b'@' {
                count += 1;
                if count >= 4 {
                    continue;
                }
            }
            let ur = i - stride + 1;
            if ur >= 0 && unsafe { *data.get_unchecked(ur as usize) } == b'@' {
                count += 1;
                if count >= 4 {
                    continue;
                }
            }
            let dl = i + stride - 1;
            if dl < len && unsafe { *data.get_unchecked(dl as usize) } == b'@' {
                count += 1;
                if count >= 4 {
                    continue;
                }
            }
            let d = i + stride;
            if d < len && unsafe { *data.get_unchecked(d as usize) } == b'@' {
                count += 1;
                if count >= 4 {
                    continue;
                }
            }
            let dr = i + stride + 1;
            if dr < len && unsafe { *data.get_unchecked(dr as usize) } == b'@' {
                count += 1;
                if count >= 4 {
                    continue;
                }
            }
            result += 1;
        }
    }
    result
}

pub fn part_two(data: &mut [u8]) -> u64 {
    let stride = data.iter().position(|&b| b == b'\n').unwrap() as isize + 1;
    let len = data.len() as isize;
    let mut result = 0;
    let mut removed;
    loop {
        removed = 0;
        for i in 0..data.len() {
            let b = unsafe { *data.get_unchecked(i) };
            let i = i as isize;
            if b == b'@' {
                let mut count: u8 = 0;
                let l = i - 1;
                if l >= 0 && unsafe { *data.get_unchecked(l as usize) } == b'@' {
                    count += 1;
                }
                let r = i + 1;
                if r < len && unsafe { *data.get_unchecked(r as usize) } == b'@' {
                    count += 1;
                }
                let ul = i - stride - 1;
                if ul >= 0 && unsafe { *data.get_unchecked(ul as usize) } == b'@' {
                    count += 1;
                }
                let u = i - stride;
                if u >= 0 && unsafe { *data.get_unchecked(u as usize) } == b'@' {
                    count += 1;
                    if count >= 4 {
                        continue;
                    }
                }
                let ur = i - stride + 1;
                if ur >= 0 && unsafe { *data.get_unchecked(ur as usize) } == b'@' {
                    count += 1;
                    if count >= 4 {
                        continue;
                    }
                }
                let dl = i + stride - 1;
                if dl < len && unsafe { *data.get_unchecked(dl as usize) } == b'@' {
                    count += 1;
                    if count >= 4 {
                        continue;
                    }
                }
                let d = i + stride;
                if d < len && unsafe { *data.get_unchecked(d as usize) } == b'@' {
                    count += 1;
                    if count >= 4 {
                        continue;
                    }
                }
                let dr = i + stride + 1;
                if dr < len && unsafe { *data.get_unchecked(dr as usize) } == b'@' {
                    count += 1;
                    if count >= 4 {
                        continue;
                    }
                }
                data[i as usize] = b'x';
                removed += 1
            }
        }
        if removed == 0 {
            break;
        }
        result += removed;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_bytes!("testdata/printing_department/example");
        let result = part_one(data);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_one_input() {
        let data = include_bytes!("testdata/printing_department/input");
        let result = part_one(data);
        assert_eq!(result, 1523);
    }

    #[test]
    fn part_two_example() {
        let src = include_bytes!("testdata/printing_department/example");
        let mut data = [0u8; 120];
        data[..src.len()].copy_from_slice(src);
        let result = part_two(&mut data);
        assert_eq!(result, 43);
    }

    #[test]
    fn part_two_input() {
        let src = include_bytes!("testdata/printing_department/input");
        let mut data = [0u8; 20_000];
        data[..src.len()].copy_from_slice(src);
        let result = part_two(&mut data);
        assert_eq!(result, 9290);
    }
}
