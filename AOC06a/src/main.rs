fn main() {
    let input = include_str!("../input").chars().collect::<Vec<char>>();
    for i in 0..input.len()-4 {
        let marker = &input[i..i+4];
        if !marker[1..4].contains(&marker[0]) && !marker[2..4].contains(&marker[1]) && !(marker[2] == marker[3]) {
            println!("{}", i+4);
            break;
        }
    }
}
