fn main() {
    let input = include_str!("../input").chars().collect::<Vec<char>>();
    'outer: for i in 0..input.len()-4 {
        let marker = &input[i..i+14];
        for j in 0..13 {
            if marker[j+1..14].contains(&marker[j]) {
                continue 'outer;
            }
        }
        println!("{}", i+14);
        break;
    }
}