fn main() {
    let inputs = include_str!("../input").split("\n").collect::<Vec<&str>>();
    let mut count = 0;
    for input in inputs {
        let pair = input.split(",").collect::<Vec<&str>>();
        let (p1, p2) = (pair[0].split("-").collect::<Vec<&str>>(),pair[1].split("-").collect::<Vec<&str>>());
        let (a, b, c, d): (i32, i32, i32, i32) = (p1[0].parse().unwrap(),p1[1].parse().unwrap(),p2[0].parse().unwrap(),p2[1].parse().unwrap());
        if (a<=c && b>=d) || (a>=c && b<=d) {
            count += 1;
        }
    }
    println!("{}",count);
}
