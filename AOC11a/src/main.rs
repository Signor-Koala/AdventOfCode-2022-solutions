
struct Monkey {
    items: Vec<i32>,
    inspect: Box<dyn Fn(i32) -> i32>,
    div_test: (i32, i32, i32),
    times: i32,
}

impl Monkey {
    fn inspect(&mut self) -> Vec<(i32, i32)> {
        let mut order: Vec<(i32, i32)> = Vec::new();
        while let Some(i) = self.items.pop() {
            let mut x: i32 = (&self.inspect)(i).into();
            x = x/3;
            if x % self.div_test.0 == 0 {
                order.push((x, self.div_test.1))
            } else {
                order.push((x, self.div_test.2))
            }
            self.times += 1;
        }
        order
    }
}

fn main() {
    let inputs = include_str!("../input").split("\n\n");
    let mut monkeys: Vec<Monkey> = Vec::new();

    for input in inputs {
        let mut s = input.split("\n");
        s.next();
        let (_, items) = s.next().unwrap().split_once(":").unwrap();
        let items: Vec<i32> = items.split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();

        let (_, f) = s.next().unwrap().split_once("=").unwrap();
        let mut f = f.trim().split(" ");
        f.next();
        let op = f.next().unwrap();
        let val = f.next().unwrap();

        let div = s.next().unwrap().rsplit(" ").next().unwrap().parse::<i32>().unwrap();
        let true_monkey = s.next().unwrap().rsplit(" ").next().unwrap().parse::<i32>().unwrap();
        let false_monkey = s.next().unwrap().rsplit(" ").next().unwrap().parse::<i32>().unwrap();

        match op {
            "*" => {
                match val {
                    "old" => {
                        monkeys.push(Monkey{items, inspect: Box::new(|x| x*x), div_test: (div, true_monkey, false_monkey, ), times: 0});
                    },
                    s => {
                        let s = s.parse::<i32>().unwrap();
                        let f = move |x| x * s;
                        monkeys.push(Monkey{items, inspect: Box::new(f), div_test: (div, true_monkey, false_monkey, ), times: 0});
                    },
                }
            },
            "+" => {
                match val {
                    "old" => {
                        monkeys.push(Monkey { items, inspect: Box::new(|x| x + x), div_test: (div, true_monkey, false_monkey, ), times: 0 });
                    },
                    s => {
                        let s = s.parse::<i32>().unwrap();
                        let f = move |x| x + s;
                        monkeys.push(Monkey { items, inspect: Box::new(f), div_test: (div, true_monkey, false_monkey, ), times: 0 });
                    },
                }
            },
            _ => panic!(),
        }
    }
    for _ in 0..20 {
        for i in 0..8 {
            let order = monkeys[i].inspect().clone();
            for (val, j) in order {
                monkeys[j as usize].items.push(val);
            }
        }
    }
    let mut monkey_time: Vec<i32> = Vec::new();
    for monkey in &mut monkeys {
        monkey_time.push(monkey.times);
    }
    monkey_time.sort();
    monkey_time.reverse();
    println!("{:#?}", monkey_time[0] * monkey_time[1]);
}
