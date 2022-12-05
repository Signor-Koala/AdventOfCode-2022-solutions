fn main() {
    let inputs = include_str!("../input").split("\n").collect::<Vec<&str>>();
    let mut sep = 0;
    let mut columns = 0;

    for input in &inputs {
        sep += 1;
        if input.chars().nth(1).unwrap() == '1' { break }
    }

    let inputc = &inputs.clone();
    for input in inputc.clone()[sep-1].split(" ") {
        if let Ok(i) = input.parse::<i32>() {
            columns = i;
        }
    }

    println!("{}", columns);
    let mut stacks :Vec<Vec<char>> = Vec::new();
    for i in 0..columns {
        let mut col :Vec<char> = Vec::new();
        for j in (0..(sep-1)).rev() {
            let c = inputs[j].chars().nth(4*(i as usize) + 1).unwrap();
            if c == ' ' { break }
            col.push(c);
        }
        stacks.push(col);
    }

    let mut inputs = inputs.into_iter();
    for i in 0..sep+1 {
        inputs.next();
    }

    for input in inputs {
        let mut input :Vec<&str> = input.split(" ").collect();
        let (num, col1, col2) :(i32, i32, i32) = (
            input[1].parse().unwrap(),
            input[3].parse().unwrap(),
            input[5].parse().unwrap()
        );

        for i in 0..num {
            let c = stacks[col1 as usize - 1].pop().unwrap();
            stacks[col2 as usize - 1].push(c);
        }
    }

    for stack in stacks {
        print!("{}",stack.last().unwrap());
    }
}
