fn main() {
    let inputs = include_str!("../input").split("\n");
    let mut grid = vec![vec!["."; 200]; 400];
    let mut lowest_point = 0;
    for input in inputs {
        let mut points = input.split(" -> ");
        let (startx, starty) = points.next().unwrap().split_once(",").unwrap();
        let (mut startx, mut starty): (i32, i32) = (startx.parse::<i32>().unwrap() - 300, starty.parse().unwrap());
        grid[startx as usize][starty as usize] = "█";
        while let Some(s) = points.next() {
            let (x, y) = s.split_once(",").unwrap();
            let (x, y): (i32, i32) = (x.parse::<i32>().unwrap() - 300, y.parse().unwrap());
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
            if y > lowest_point { lowest_point = y }
        }
    }
    for v in &mut grid {
        v[lowest_point as usize + 2] = "_";
    }

    let mut count = 0;
    'outer: loop {
        let (mut x, mut y) = (200, 0);
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
            if (x, y) == (200, 0) {
                break 'outer;
            }
            break;
        }
    }

    /*
    for i in 0..200 {
        for j in 0..400 {
            print!("{}", grid[j][i]);
        }
        println!()
    }
    */

    println!("{}", count);
}
