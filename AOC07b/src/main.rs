extern crate core;

use std::collections::HashMap;
use regex::RegexSet;

fn main() {
    let inputs = include_str!("../input").split("\n");
    let mut directories = vec![String::from("/")];
    let mut sizes: HashMap<String, i32> = HashMap::new();
    sizes.insert(String::from("/"), 0);
    let mut pwd = String::from("/");

    let set = RegexSet::new(&[
        r"\$ cd /",
        r"\$ cd \.\.",
        r"\$ cd ([a-zA-Z]+)",
        r"\$ ls",
        r"dir ([a-zA-Z]+)",
        r"\d+\s.+",
    ]).unwrap();

    for input in inputs {
        let found: Vec<_> = set.matches(input).into_iter().collect();
        match found.last().unwrap() {
            0 => {
                pwd = String::from("/");
            },
            1 => {
                let (temp, _) = pwd.rsplit_once("/").unwrap();
                let (temp, _) = temp.rsplit_once("/").unwrap();
                if temp == "" {
                    pwd = String::from("/");
                } else {
                    pwd = String::from(&format!("{}/",temp));
                }
            },
            2 => {
                pwd.push_str(&format!("{}/",input.split(" ").last().unwrap()));
            }
            3 => {
            },
            4 => {
                let dir = input.split(" ").last().unwrap();
                if !directories.contains(&String::from(dir)) {
                    let mut temp = pwd.clone();
                    temp.push_str(&format!("{}/",dir));
                    sizes.insert(String::from(&temp), 0);
                    directories.push(temp);
                }
            },
            5 => {
                let mut split = input.split(" ");
                let size: i32 = split.next().unwrap().parse().unwrap();
                let _name = split.next().unwrap();
                for directory in &directories {
                    if pwd.contains(directory) {
                        let temp = sizes.get(directory).unwrap() + size;
                        sizes.insert(String::from(directory), temp);
                    }
                }
            }
            _ => panic!()
        };
    }

    let mut output = *sizes.get("/").unwrap();
    let needed =  output - 70000000 + 30000000;
    for (name, size) in sizes {
        if size > needed && size < output {
            output = size;
        }
    }
    println!("{}", output);
}
