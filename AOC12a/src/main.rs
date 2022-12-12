fn main() {
    let (mut grid, pos) = get_input();
    let mut positions = vec![(pos, 0)];
    let cond = |height: char, s: char| {
        (s.is_lowercase() && ((s as u8 <= height as u8) || (s as u8).abs_diff(height as u8) <= 1)) || (s == 'E' && height == 'z') || (height == 'S' && s as u8 <= b'b')
    };
    let mut totcount = 0;
    while let Some((pos, count)) = &mut positions.pop() {
        if grid[pos.0][pos.1].0 == 'E' {
            totcount = *count;
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
    println!("{}", totcount);
}

fn get_input() -> (Vec<Vec<(char, bool)>>, (usize, usize)) {
    let inputs = include_str!("../input").split("\n");
    let mut grid: Vec<Vec<(char, bool)>> = Vec::new();
    let mut pos: (usize, usize) = (0, 0);
    let (mut i, mut j) = (1, 0);
    for input in inputs {
        let mut row: Vec<(char, bool)> = Vec::new();
        row.push(('|', false));
        j = 1;
        for char in input.chars() {
            match char {
                'S' => {
                    pos = (i, j);
                    row.push(('S',true));
                },
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