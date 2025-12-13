pub fn part_one(data: &str) -> u64 {
    calc_pwd(data, false)
}

pub fn part_two(data: &str) -> u64 {
    calc_pwd(data, true)
}

fn calc_pwd(data: &str, newer_method: bool) -> u64 {
    let mut result: u64 = 0;
    let mut cur: i64 = 50;
    for line in data.lines() {
        let (op, s) = line.split_at(1);
        let val = s.parse::<u64>().ok().unwrap();
        let step = (val % 100) as i64;
        debug_print!("{} {}{}", cur, op, val);
        if newer_method {
            let r = val / 100;
            if r > 0 {
                result += r;
                debug_print!("r: {}", r)
            }
        }
        let num = if op == "L" { cur - step } else { cur + step };
        debug_print!(" -> {}", num);
        if num == 0 {
            result += 1;
            cur = 0;
        } else if num == 100 {
            result += 1;
            cur = 0;
        } else if num < 0 {
            if newer_method && cur != 0 {
                result += 1;
                debug_print!(" rm: {}", 1)
            }
            cur = num + 100;
        } else if num > 100 {
            if newer_method {
                result += 1;
                debug_print!(" rp: {}", 1)
            }
            cur = num - 100;
        } else {
            cur = num;
        }
        debug_println!(" final: {} {}", cur, result)
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_str!("testdata/secret_entrance/example");
        let result = part_one(data);
        assert_eq!(result, 3);
    }

    #[test]
    fn part_one_input() {
        let data = include_str!("testdata/secret_entrance/input");
        let total = part_one(data);
        assert_eq!(total, 1081);
    }

    #[test]
    fn part_two_example() {
        let data = include_str!("testdata/secret_entrance/example");
        let result = part_two(data);
        assert_eq!(result, 6);
    }

    #[test]
    fn part_two_input() {
        let data = include_str!("testdata/secret_entrance/input");
        let total = part_two(data);
        assert_eq!(total, 6689);
    }
}
