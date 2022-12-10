fn main() {
    let mut inputs = include_str!("../input").split("\n");
    let (mut stack, mut j, mut str, mut ans) = (0, 0, 1, 0);
    for i in 0..220 {
        if j == 0 {
            str += stack;
            stack = 0;
            println!("{:#?}\t{:#?}", i+1, str);
            match inputs.next().unwrap() {
                "noop" => j += 1,
                s => (j, stack) = (2, s.rsplit_once(" ").unwrap().1.parse::<i32>().unwrap())
            }
        }
        j -= 1;
        if (i+1) % 40 == 20 {
            println!("{:#?}\t{:#?}", i+1, str);
            ans += (i+1)*str;
        }
    }

    println!("{:#?}", ans);
}
