fn main() {
    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let inputs :Vec<&str> = include_str!("../input.txt").split("\n").collect();
    let mut inputs = inputs.into_iter();
    let mut p = 0;
    loop {
        let l1 = inputs.next();
        if let None = l1 {
            break
        }
        let l1 = l1.unwrap();
        let l2 = inputs.next().unwrap();
        let l3 = inputs.next().unwrap();
        for c in l1.chars() {
            if let Some(_) = l2.find(c){
                if let Some(_) = l3.find(c){
                    p += priorities.find(c).unwrap() + 1;
                    break;
                }
            }
        }
        println!("{}",p);
    }

    println!("{}",p);
}