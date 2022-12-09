use std::collections::HashMap;

fn main() {
    let inputs = include_str!("../input").split("\n");
    let mut visited: HashMap<(i32,i32), bool> = HashMap::from([((0,0), true)]);
    let (mut posH, mut posT): ((i32, i32), (i32, i32)) = ((0, 0), (0, 0));
    for input in inputs.map(|s| {
        let mut s = s.split(" ");
        (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap())
    }) {
        for _ in 0..input.1 {
            let move_rope = |x: &mut (i32, i32)|{
                match input.0 {
                    "U" => x.1 += 1,
                    "D" => x.1 -= 1,
                    "R" => x.0 += 1,
                    "L" => x.0 -= 1,
                    _ => panic!(),
                }
            };
            println!("{}",input.0);
            move_rope(&mut posH);
            if posH.0.abs_diff(posT.0) > 1 || posH.1.abs_diff(posT.1) > 1 {
                move_rope(&mut posT);
                if posT.0 != posH.0 && posT.1 != posH.1 {
                    match input.0 {
                        "U" | "D" => posT.0 += (posH.0 - posT.0).signum(),
                        "R" | "L" => posT.1 += (posH.1 - posT.1).signum(),
                        _ => panic!(),
                    }
                }
            }
            visited.insert((posT.0, posT.1), true);
        }
    }
    println!("{:#?}", visited.len());
}
