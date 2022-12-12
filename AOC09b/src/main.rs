use std::collections::HashMap;

fn main() {
    let inputs = include_str!("../input").split("\n");
    let mut visited: HashMap<(i32,i32), bool> = HashMap::from([((0,0), true)]);
    let mut rope: [(i32, i32); 10] = [(0, 0); 10];
    for input in inputs.map(|s| {
        let mut s = s.split(" ");
        (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap())
    }) {
        for _ in 0..input.1 {
            match input.0 {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                _ => panic!(),
            }

            for i in 1..10 {
                if rope[i-1].0.abs_diff(rope[i].0) > 1 || rope[i-1].1.abs_diff(rope[i].1) > 1 {
                    rope[i].0 += (rope[i-1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i-1].1 - rope[i].1).signum();
                }
            }
            visited.insert((rope[9].0, rope[9].1), true);
        }
    }
    println!("{:#?}", visited.len());
}
