fn main() {
    let input = include_str!("../input")
        .split("\n").collect::<Vec<&str>>();
    let mut soln = 0;
    for val in input {
        let (a, b) = ((val.bytes().nth(0).unwrap() - b'A') as i32 + 1, (val.bytes().nth(2).unwrap() - b'X') as i32 + 1);
        soln += b;
        if a.abs_diff(b) == 2 {
            soln += ((a - b)/2 + 1)*3;
        } else {
            soln += (b - a + 1)*3;
        }
        println!("{}", soln);
    }
}
