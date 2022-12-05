fn main() {
    let input = include_str!("../input")
        .split("\n").collect::<Vec<&str>>();
    let mut soln = 0;
    for val in input {
        let (a, b) = ((val.bytes().nth(0).unwrap() - b'A') as i32 + 1, (val.bytes().nth(2).unwrap() - b'X') as i32 + 1);
        soln += (b-1)*3;
        let y = a + b - 2;
        soln += match y {
            0 => 3,
            4 => 1,
            _ => y,
        };

        println!("{}", soln);
    }
}