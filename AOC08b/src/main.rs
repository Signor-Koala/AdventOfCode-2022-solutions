fn main() {
    let inputs: Vec<&str> = include_str!("../input").split("\n").collect();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for input in inputs {
        let mut row: Vec<u8> = Vec::new();
        let input: Vec<u8> = input.bytes().collect();
        for i in input { row.push(i as u8 - b'0') }
        grid.push(row);
    }
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let (mut west,mut east,mut north,mut south) = (0,0,0,0);
            for k in (0..i).rev() {
                if grid[k][j] < grid[i][j] {
                    north += 1;
                    continue
                }
                north += 1;
                break;
            }
            for k in i+1..grid.len()-1 {
                if grid[k][j] < grid[i][j] {
                    south += 1;
                    continue
                }
                south += 1;
                break;
            }
            for k in (0..j).rev() {
                if grid[i][k] < grid[i][j] {
                    east += 1;
                    continue
                }
                east += 1;
                break;
            }
            for k in j+1..grid[i].len() {
                if grid[i][k] < grid[i][j] {
                    west += 1;
                    continue
                }
                west += 1;
                break;
            }
            if count < west*east*north*south {
                count = west*east*north*south;
            }
        }
    }
    println!("{}",count);
}
