pub fn part_one(data: &str, max_connections: usize) -> u64 {
    let mut boxes = parse_boxes(data);
    let distances = calc_distances(&boxes);
    let mut circuit = 1;
    for (i, j, _) in distances.iter().take(max_connections) {
        let x = boxes[*i];
        let y = boxes[*j];
        if x.3 == 0 && y.3 == 0 {
            debug_println!("Connect X: {:?} and Y: {:?} both to {}", x, y, circuit);
            boxes[*i].3 = circuit;
            boxes[*j].3 = circuit;
            circuit += 1;
        } else if x.3 == y.3 {
            debug_println!("X: {:?} and Y: {:?} are already connected", x, y);
            continue;
        } else if x.3 == 0 {
            debug_println!("Connect X: {:?} to Y circuit: {:?}", x, y);
            boxes[*i].3 = y.3;
        } else if y.3 == 0 {
            debug_println!("Connect Y: {:?} to X circuit: {:?}", y, x);
            boxes[*j].3 = x.3;
        } else {
            for k in 0..boxes.len() {
                if boxes[k].3 == y.3 {
                    boxes[k].3 = x.3;
                }
            }
        }
    }
    let mut circuits: Vec<u64> = vec![0; circuit as usize];
    for b in boxes.iter() {
        if b.3 > 0 {
            circuits[b.3 as usize] += 1
        }
    }
    circuits.sort_unstable_by(|a, b| b.cmp(a));
    circuits[0] * circuits[1] * circuits[2]
}

pub fn part_two(data: &str) -> u64 {
    let mut boxes = parse_boxes(data);
    let distances = calc_distances(&boxes);
    let mut circuit = 1;
    for (i, j, _) in distances.iter() {
        let x = boxes[*i];
        let y = boxes[*j];
        if x.3 == 0 && y.3 == 0 {
            boxes[*i].3 = circuit;
            boxes[*j].3 = circuit;
            circuit += 1;
        } else if x.3 == y.3 {
            continue;
        } else {
            let mut short = false;
            let target = if x.3 == 0 {
                boxes[*i].3 = y.3;
                short = true;
                y.3
            } else if y.3 == 0 {
                boxes[*j].3 = x.3;
                short = true;
                x.3
            } else {
                x.3
            };
            let mut same = true;
            for k in 0..boxes.len() {
                if !short && boxes[k].3 == y.3 {
                    boxes[k].3 = x.3;
                }
                if same && boxes[k].3 != target {
                    same = false;
                    if short {
                        break;
                    }
                }
            }
            if same {
                return x.0 as u64 * y.0 as u64;
            }
        }
    }
    0
}

fn parse_boxes(data: &str) -> Vec<(isize, isize, isize, u64)> {
    let mut boxes = Vec::new();
    for line in data.lines() {
        let mut coords = line.split(',');
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        let z = coords.next().unwrap().parse().unwrap();
        boxes.push((x, y, z, 0));
    }
    boxes
}

fn calc_distances(boxes: &Vec<(isize, isize, isize, u64)>) -> Vec<(usize, usize, u64)> {
    let mut distances = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let x = boxes[i];
            let y = boxes[j];
            let dx = x.0.abs_diff(y.0) as u64;
            let dy = x.1.abs_diff(y.1) as u64;
            let dz = x.2.abs_diff(y.2) as u64;
            let distance_sq = dx * dx + dy * dy + dz * dz;
            distances.push((i, j, distance_sq));
        }
    }
    distances.sort_unstable_by_key(|a| a.2);
    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_str!("testdata/playground/example");
        let result = part_one(data, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn part_one_input() {
        let data = include_str!("testdata/playground/input");
        let total = part_one(data, 1000);
        assert_eq!(total, 171503);
    }

    #[test]
    fn part_two_example() {
        let data = include_str!("testdata/playground/example");
        let result = part_two(data);
        assert_eq!(result, 25272);
    }

    #[test]
    fn part_two_input() {
        let data = include_str!("testdata/playground/input");
        let total = part_two(data);
        assert_eq!(total, 9069509600);
    }
}
