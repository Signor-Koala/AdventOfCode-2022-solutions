use std::cmp::Ordering;

fn main() {
    let inputs = include_str!("../input").split("\n").filter(|s|*s != "");
    let mut santized_inputs = Vec::new();
    for input in inputs {
        let mut inp = String::new();
        for c in input.chars() {
            match c {
                '[' | ']' => {
                    inp.push(',');
                    inp.push(c);
                    inp.push(',');
                },
                o => inp.push(o),
            }
        }
        santized_inputs.push(inp);
    }
    santized_inputs.sort_by(|a, b|compare_packets(a, b));
    let mut answer = 1;
    for i in 0.. {
        if compare_packets(&String::from(",[,[,2,],],"), &santized_inputs[i]) == Ordering::Less {
            answer *= i+1;
            break;
        }
    }
    for i in 0.. {
        if compare_packets(&String::from(",[,[,6,],],"), &santized_inputs[i]) == Ordering::Less {
            answer *= i+2;
            break;
        }
    }
    println!("{}", answer);
}

fn compare_packets(left: &String, right: &String) -> Ordering {
    let (mut left, mut right): (Vec<&str>, Vec<&str>) = (left.split(",").filter(|s| *s != "").collect(), right.split(",").filter(|s| *s != "").collect());

    let (mut il, mut ir): (usize, usize) = (0, 0);
    loop {
        if left[il] == "]" && right[ir] != "]" {
            return Ordering::Less
        }
        if left[il] != "]" && right[ir] == "]" {
            return Ordering::Greater
        }

        if left[il] == "[" && right[ir] != "[" {
            right.insert(ir+1, "]");
            il += 1;
            continue;
        }
        if left[il] != "[" && right[ir] == "[" {
            left.insert(il+1, "]");
            ir += 1;
            continue;
        }

        if let Ok(l) = left[il].parse::<i32>() {
            match l.cmp(&right[ir].parse::<i32>().unwrap()) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => {},
            }
        }

        il += 1;
        ir += 1;
    }
}