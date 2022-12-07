extern crate core;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::TNode::Dir;
use regex::{Regex, RegexSet};

enum TNode {
    File {
        size: i32,
        name: String,
    },
    Dir(HashMap<String,TNode>),
}

/*
impl TNode {

    fn cdForward(&mut self, key: &str) -> &mut Self {
        let temp = match self {
            TNode::Dir(m) => m.get(key).unwrap().as_mut(),
            _ => panic!(),
        };
        temp
    }

    fn insertDir(self, name: &str) {
        let mut newDir = TNode::Dir(HashMap::new());
        match self {
            TNode::Dir(mut m) => m.insert(String::from(name), Box::new(newDir)).unwrap(),
            _ => panic!(),
        };
    }

    fn insertFile(self, size: i32, name: &str) {
        let mut newFile = TNode::File {size, name: String::from(name)};
        match self {
            TNode::Dir(mut m) => m.insert(String::from(name), Box::new(newFile)).unwrap(),
            _ => panic!(),
        };
    }
}
*/
fn main() {
    let inputs = include_str!("../input").split("\n");
    let mut root = TNode::Dir(HashMap::new());

    let set = RegexSet::new(&[
        r"$ cd /",
        r"$ cd \.\.",
        r"$ cd ([a-zA-Z]+)",
        r"$ ls",
        r"dir ([a-zA-Z]+)",
        r"\d+ \w+",
    ]).unwrap();

    let mut pwd = vec![Box::new(root)];
    for input in inputs {
        let found: Vec<_> = set.matches(input).into_iter().collect();
        match found.last().unwrap() {
            0 => {
                while pwd.len() != 1 {
                    pwd.pop().unwrap();
                }
            },
            1 => {
                pwd.pop().unwrap();
            },
            2 => {
                let mut cwd = pwd.pop().unwrap();
                let name = input.split(" ").last().unwrap();
                let mut nwd = match cwd.as_mut() {
                    TNode::Dir(m) => m.get_mut(name).unwrap(),
                    _ => panic!(),
                };
                pwd.push(cwd);
                pwd.push(nwd);
                // let temp = pwd.last().as_mut().unwrap().cdForward(input.split(" ").last().unwrap());
                // pwd.push(temp);
            }
            3 => {},
            _ => {},





        };

    }
}
