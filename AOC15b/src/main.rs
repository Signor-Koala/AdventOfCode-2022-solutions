use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut inputs = include_str!("../input").lines();
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let mut sensors = Vec::new();
    for input in inputs {
        let cap = re.captures(input).unwrap();
        let ((x, y), (xb, yb)): ((i32, i32), (i32, i32)) = (
            (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            (cap[3].parse().unwrap(), cap[4].parse().unwrap()));
        let range = x.abs_diff(xb) as i32 + y.abs_diff(yb) as i32;
        sensors.push(Sensor{x, y, range});
    }
    let mut check_space = Vec::new();
    for s in &sensors {
        check_space.push((s.x - s.range - 1, s.y));
        for x in (s.x - s.range)..=(s.x + s.range) {
            check_space.push((x, s.y + ((x.abs_diff(s.x) as i32).abs_diff(s.range) as i32) + 1));
            check_space.push((x, s.y - ((x.abs_diff(s.x) as i32).abs_diff(s.range) as i32) - 1));
        }
        check_space.push((s.x + s.range + 1, s.y));
    }
    check_space = check_space.into_iter().filter(|(x, y)| 0 <= *x && *x<= 4000000 && 0 <= *y && *y <= 4000000).collect();

    'outer: for p in check_space {
        for sensor in &sensors {
            if (p.0.abs_diff(sensor.x) + p.1.abs_diff(sensor.y)) <= sensor.range as u32 {
                continue 'outer;
            }
        }
        println!("{:?}", (p.0 as i64 * 4000000) + p.1 as i64);
        break;
    }
}

struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}
