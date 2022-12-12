fn main() {
    let (grid_og, starting_points) = get_input();
    let mut total_counts: Vec<i32> = Vec::new();

    for starting_point in starting_points {
        let mut grid = grid_og.clone();
        let mut positions = vec![(starting_point, 0)];
        let cond = |height: char, s: char| {
            (s.is_lowercase() && ((s as u8 <= height as u8) || (s as u8).abs_diff(height as u8) <= 1)) || (s == 'E' && height == 'z') || (height == 'S' && s as u8 <= b'b')
        };
        while let Some((pos, count)) = &mut positions.pop() {
            if grid[pos.0][pos.1].0 == 'E' {
                total_counts.push(*count);
                break;
            }
            if let (s, false) = grid[pos.0][pos.1 + 1] {
                if cond(grid[pos.0][pos.1].0, s) {
                    positions.insert(0,((pos.0, pos.1 + 1), *count + 1));
                    grid[pos.0][pos.1 + 1] = (s, true);
                }
            }
            if let (s, false) = grid[pos.0 + 1][pos.1] {
                if cond(grid[pos.0][pos.1].0, s) {
                    positions.insert(0,((pos.0 + 1, pos.1), *count + 1));
                    grid[pos.0 + 1][pos.1] = (s, true);
                }
            }
            if let (s, false) = grid[pos.0][pos.1 - 1] {
                if cond(grid[pos.0][pos.1].0, s) {
                    positions.insert(0,((pos.0, pos.1 - 1), *count + 1));
                    grid[pos.0][pos.1 - 1] = (s, true);
                }
            }
            if let (s, false) = grid[pos.0 - 1][pos.1] {
                if cond(grid[pos.0][pos.1].0, s) {
                    positions.insert(0,((pos.0 - 1, pos.1), *count + 1));
                    grid[pos.0 - 1][pos.1] = (s, true);
                }
            }
        }
    }
    total_counts.sort();
    println!("{:?}", total_counts[0]);
}

fn get_input() -> (Vec<Vec<(char, bool)>>, Vec<(usize, usize)>) {
    let inputs = include_str!("../input").split("\n");
    let mut grid: Vec<Vec<(char, bool)>> = Vec::new();
    let mut pos: Vec<(usize, usize)> = Vec::new();
    let (mut i, mut j) = (1, 0);
    for input in inputs {
        let mut row: Vec<(char, bool)> = Vec::new();
        row.push(('|', false));
        j = 1;
        for char in input.chars() {
            match char {
                'S' => {
                    pos.push((i, j));
                    row.push(('S',true));
                },
                'a' => {
                    pos.push((i, j));
                    row.push(('a', false))
                }
                s => row.push((s, false)),
            }
            j += 1;
        }
        i += 1;
        row.push(('|', false));
        j += 1;
        grid.push(row);
    }
    let (mut edge1, mut edge2): (Vec<(char, bool)>, Vec<(char, bool)>,) = (Vec::new(), Vec::new());
    for _ in 0..j {
        edge1.push(('|', false));
        edge2.push(('|', false));
    }
    grid.insert(0, edge1);
    grid.push(edge2);
    (grid, pos)
}
