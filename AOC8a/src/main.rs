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
            let (mut west,mut east,mut north,mut south) = (true,true,true,true);
            for k in 0..i {
                if grid[i][j] <= grid[k][j] {
                    north = false;
                    break;
                }
            }
            for k in i+1..grid.len() {
                if grid[i][j] <= grid[k][j] {
                    south = false;
                    break;
                }
            }
            for k in 0..j {
                if grid[i][j] <= grid[i][k] {
                    west = false;
                    break;
                }
            }
            for k in j+1..grid[i].len() {
                if grid[i][j] <= grid[i][k] {
                    east = false;
                    break;
                }
            }
            if west || east || north || south {
                count += 1;
            }
        }
    }
}
