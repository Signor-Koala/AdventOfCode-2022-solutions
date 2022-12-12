fn not_main() {
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
    searchPath(pos, &mut grid, 0);
}

fn searchPath(pos: (usize, usize), m: &mut Vec<Vec<(char, bool)>>, mut c: i32) {
    let (height, _)= m[pos.0][pos.1];
    if height == 'E' {
        println!("{}", c);
        for row in m {
            for (_, b) in row {
                if *b {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        return;
    }
    c += 1;
    m[pos.0][pos.1] = (height, true);

    if let (s, false) = m[pos.0][pos.1 + 1] {
        if (s.is_lowercase() && ((s as u8 <= height as u8) || (s as u8).abs_diff(height as u8) <= 1)) || (s == 'E' && height == 'z') || (height == 'S' && s as u8 <= b'b') {
            searchPath((pos.0, pos.1 + 1), m, c);
        }
    }
    if let (s, false) = m[pos.0 + 1][pos.1] {
        if (s.is_lowercase() && ((s as u8 <= height as u8) || (s as u8).abs_diff(height as u8) <= 1)) || (s == 'E' && height == 'z') || (height == 'S' && s as u8 <= b'b') {
            searchPath((pos.0 + 1, pos.1), m, c);
        }
    }
    if let (s, false) = m[pos.0][pos.1 - 1] {
        if (s.is_lowercase() && ((s as u8 <= height as u8) || (s as u8).abs_diff(height as u8) <= 1)) || (s == 'E' && height == 'z') || (height == 'S' && s as u8 <= b'b') {
            searchPath((pos.0, pos.1 - 1), m, c);
        }
    }
    if let (s, false) = m[pos.0 - 1][pos.1] {
        if (s.is_lowercase() && ((s as u8 <= height as u8) || (s as u8).abs_diff(height as u8) <= 1)) || (s == 'E' && height == 'z') || (height == 'S' && s as u8 <= b'b') {
            searchPath((pos.0 - 1, pos.1), m, c);
        }
    }
    m[pos.0][pos.1] = (height, false);
}
