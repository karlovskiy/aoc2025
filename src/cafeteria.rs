pub fn part_one(data: &str) -> u64 {
    let mut result: u64 = 0;
    let mut searching = false;
    let mut ranges: Vec<u64> = vec![];
    for line in data.lines() {
        if line == "" {
            searching = true;
            debug_println!("{:?}", ranges);
            continue;
        }
        if searching {
            let num = line.parse::<u64>().unwrap();
            let len = ranges.len();
            let mut i = 0;
            while i < len - 1 {
                unsafe {
                    let start = *ranges.get_unchecked(i);
                    let end = *ranges.get_unchecked(i + 1);
                    if start <= num && num <= end {
                        result += 1;
                        break;
                    }
                    i += 2;
                }
            }
        } else {
            let (start, end) = line.split_once('-').unwrap();
            ranges.push(start.parse::<u64>().unwrap());
            ranges.push(end.parse::<u64>().unwrap());
        }
    }
    result
}

pub fn part_two(data: &str) -> u64 {
    let mut ranges: Vec<Range> = vec![];
    for line in data.lines() {
        if line == "" {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        ranges.push(Range { start, end });
    }
    if ranges.is_empty() {
        return 0;
    }
    ranges.sort_unstable_by_key(|r| r.start);
    let mut result = 0;
    let mut iter = ranges.into_iter();
    let first = iter.next().unwrap();
    let mut cur_start = first.start;
    let mut cur_end = first.end;
    for x in iter {
        if x.start > cur_end {
            result += cur_end - cur_start + 1;
            cur_start = x.start;
            cur_end = x.end;
        } else {
            if x.end > cur_end {
                cur_end = x.end;
            }
        }
    }
    result += cur_end - cur_start + 1;
    result
}

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_str!("testdata/cafeteria/example");
        let result = part_one(data);
        assert_eq!(result, 3);
    }

    #[test]
    fn part_one_input() {
        let data = include_str!("testdata/cafeteria/input");
        let result = part_one(data);
        assert_eq!(result, 739);
    }

    #[test]
    fn part_two_example() {
        let data = include_str!("testdata/cafeteria/example");
        let result = part_two(data);
        assert_eq!(result, 14);
    }

    #[test]
    fn part_two_input() {
        let data = include_str!("testdata/cafeteria/input");
        let result = part_two(data);
        assert_eq!(result, 344486348901788);
    }
}
