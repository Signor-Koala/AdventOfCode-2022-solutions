
struct Monkey {
    items: Vec<i64>,
    inspect: Box<dyn Fn(i64) -> i64>,
    div_test: (i64, i64, i64),
    times: i64,
}

impl Monkey {
    fn inspect(&mut self, div_total: i64) -> Vec<(i64, i64)> {
        let mut order: Vec<(i64, i64)> = Vec::new();
        while let Some(i) = self.items.pop() {
            let mut x: i64 = (&self.inspect)(i).into();
            if x / div_total > 0 {
                x = x % div_total;
            }
            if x as i64 % self.div_test.0 == 0 {
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
    let mut div_total: i64 = 1;

    for input in inputs {
        let mut s = input.split("\n");
        s.next();
        let (_, items) = s.next().unwrap().split_once(":").unwrap();
        let items: Vec<i64> = items.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect();

        let (_, f) = s.next().unwrap().split_once("=").unwrap();
        let mut f = f.trim().split(" ");
        f.next();
        let op = f.next().unwrap();
        let val = f.next().unwrap();

        let div = s.next().unwrap().rsplit(" ").next().unwrap().parse::<i64>().unwrap();
        let true_monkey = s.next().unwrap().rsplit(" ").next().unwrap().parse::<i64>().unwrap();
        let false_monkey = s.next().unwrap().rsplit(" ").next().unwrap().parse::<i64>().unwrap();
        div_total *= div;

        match op {
            "*" => {
                match val {
                    "old" => {
                        monkeys.push(Monkey{items, inspect: Box::new(|x| x*x), div_test: (div, true_monkey, false_monkey, ), times: 0});
                    },
                    s => {
                        let s = s.parse::<i64>().unwrap();
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
                        let s = s.parse::<i64>().unwrap();
                        let f = move |x| x + s;
                        monkeys.push(Monkey { items, inspect: Box::new(f), div_test: (div, true_monkey, false_monkey, ), times: 0 });
                    },
                }
            },
            _ => panic!(),
        }
    }
    for _ in 0..10000 {
        for i in 0..8 {
            let order = monkeys[i].inspect(div_total).clone();
            for (val, j) in order {
                monkeys[j as usize].items.push(val);
            }
        }
    }
    let mut monkey_time: Vec<i64> = Vec::new();
    for monkey in &mut monkeys {
        monkey_time.push(monkey.times);
    }
    monkey_time.sort();
    monkey_time.reverse();
    println!("{:#?}", monkey_time[0] * monkey_time[1]);
}