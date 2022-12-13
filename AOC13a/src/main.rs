use std::cmp::Ordering;

fn main() {
    let inputs = include_str!("../input").split("\n\n");
    let mut i = 1;
    let mut total = 0;
    for input in inputs {
        let (inleft, inright) = input.split_once("\n").unwrap();
        let (mut left, mut right) = (String::new(), String::new());
        for c in inleft.chars() {
            match c {
                '[' | ']' => {
                    left.push(',');
                    left.push(c);
                    left.push(',');
                },
                o => left.push(o),
            }
        }
        for c in inright.chars() {
            match c {
                '[' | ']' => {
                    right.push(',');
                    right.push(c);
                    right.push(',');
                },
                o => right.push(o)
            }
        }
        let (mut left, mut right): (Vec<&str>, Vec<&str>) = (left.split(",").filter(|s| *s != "").collect(), right.split(",").filter(|s| *s != "").collect());

        let (mut il, mut ir): (usize, usize) = (0, 0);
        loop {
            if left[il] == "]" && right[ir] != "]" {
                total += i;
                break;
            }
            if left[il] != "]" && right[ir] == "]" {
                break;
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
                    Ordering::Less => { total += i; break },
                    Ordering::Greater => break,
                    Ordering::Equal => {},
                }
            }

            il += 1;
            ir += 1;
        }
        i += 1;
    }
    println!("{}", total);
}
