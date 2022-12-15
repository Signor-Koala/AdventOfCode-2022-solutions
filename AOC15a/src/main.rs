use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut inputs = include_str!("../input").lines();
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let mut row = HashMap::new();
    let mut beacons = Vec::new();
    for input in inputs {
        let cap = re.captures(input).unwrap();
        let ((x, y), (xb, yb)): ((i32, i32), (i32, i32)) = (
            (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            (cap[3].parse().unwrap(), cap[4].parse().unwrap()));
        let range = x.abs_diff(xb) as i32 + y.abs_diff(yb) as i32;
        if (y.abs_diff(2000000) as i32) <= range {
            for xr in (x - (range - y.abs_diff(2000000) as i32))..=(x + (range - y.abs_diff(2000000) as i32)) {
                row.insert((xr, 2000000), true);
            }
        }
        beacons.push((xb, yb));
    }
    for beacon in beacons {
        row.remove(&beacon).unwrap_or(true);
    }

    println!("{}", row.len());
}
