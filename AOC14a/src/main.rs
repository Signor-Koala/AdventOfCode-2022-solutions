fn main() {
    let inputs = include_str!("../input").split("\n");
    let mut grid = vec![vec!["."; 200]; 200];
    for input in inputs {
        let mut points = input.split(" -> ");
        let (startx, starty) = points.next().unwrap().split_once(",").unwrap();
        let (mut startx, mut starty): (i32, i32) = (startx.parse::<i32>().unwrap() - 400, starty.parse().unwrap());
        grid[startx as usize][starty as usize] = "█";
        while let Some(s) = points.next() {
            let (x, y) = s.split_once(",").unwrap();
            let (x, y): (i32, i32) = (x.parse::<i32>().unwrap() - 400, y.parse().unwrap());
            if startx == x {
                while starty != y {
                    if starty > y {
                        starty -= 1;
                    } else {
                        starty += 1;
                    }
                    grid[startx as usize][starty as usize] = "█";
                }
            }
            if starty == y {
                while startx != x {
                    if startx > x {
                        startx -= 1;
                    } else {
                        startx += 1;
                    }
                    grid[startx as usize][starty as usize] = "█";
                }
            }
        }
    }

    let mut count = 0;
    'outer: loop {
        let (mut x, mut y) = (100, 0);
        loop {
            if y+1 == grid[x].len() {
                break 'outer;
            }
            if grid[x][y + 1] == "." {
                y += 1;
                continue;
            }
            if grid[x-1][y+1] == "." {
                y += 1;
                x -= 1;
                continue;
            }
            if grid[x+1][y+1] == "." {
                y += 1;
                x += 1;
                continue;
            }
            grid[x][y] = "#";
            count += 1;
            break;
        }
    }

    /*
    for i in 0..200 {
        for j in 0..200 {
            print!("{}", grid[j][i]);
        }
        println!()
    }
    */

    println!("{}", count);
}