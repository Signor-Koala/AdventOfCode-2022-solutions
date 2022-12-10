fn main() {
    let mut inputs = include_str!("../input").split("\n");
    let (mut stack, mut j, mut str) = (0, 0, 1);
    for i in 0.. {
        if j == 0 {
            str += stack;
            stack = 0;
            match inputs.next().unwrap_or_else(||"break") {
                "break" => break,
                "noop" => j += 1,
                s => (j, stack) = (2, s.rsplit_once(" ").unwrap().1.parse::<i32>().unwrap())
            }
        }
        j -= 1;
        if ((i % 40) as i32).abs_diff(str) <= 1 {
            print!("█");
        } else {
            print!(".");
        }
        if (i+1) % 40 == 0 {
            println!();
        }
    }
}
