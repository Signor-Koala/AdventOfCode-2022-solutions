fn main() {
    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let inputs: Vec<&str> = include_str!("../input.txt").split("\n").collect();
    let mut p = 0;
    for input in inputs {
        let (a, b) = input.split_at(input.len()/2);
        let a = a.chars();
        for c in a {
            if let Some(_) = b.find(c) {
                p += priorities.find(c).unwrap() + 1;
                break;
            }
        }
    }
    println!("{}",p);
}
