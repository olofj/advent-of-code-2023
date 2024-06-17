use std::collections::HashMap;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

const MOVES: &'static [(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn next(grid: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let cur = grid[x][y];
    let x = x as isize;
    let y = y as isize;
    let moves = match cur {
        '|' => vec![(1, 0), (-1, 0)],
        '-' => vec![(0, 1), (0, -1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(1, 0), (0, -1)],
        'F' => vec![(1, 0), (0, 1)],
        _ => {
            if cur != '.' { println!("Unknown move {}", cur); }
            vec![]
        }
    };
    let moves: Vec<_> = moves
        .into_iter()
        .map(|d| (x + d.0, y + d.1))
        .filter(|(x, y)| {
            //println!("        {:?} contains {:?}", (0..grid.len() as isize), (x+dx, y+dy));
            (0..grid.len() as isize).contains(&x)
                && (0..grid[0].len() as isize).contains(&y)
        })
        .map(|m| (m.0 as usize, m.1 as usize))
        .collect();
    let m = moves.iter().map(|m| (m.0 as usize, m.1 as usize)).collect::<Vec<(usize,usize)>>();
    m
}

fn reconstruct_start(grid: &Vec<Vec<char>>) -> char {
    let start: (usize, usize) = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, ch)| if ch == &'S' { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .next()
        .unwrap();

    let from_above = start.0 > 0 && ['|', '7', 'F'].contains(&grid[start.0-1][start.1]);
    let from_below = ['|', 'L', 'J'].contains(&grid[start.0+1][start.1]);
    let from_left = start.1 > 0 && ['-', 'L', 'F'].contains(&grid[start.0][start.1-1]);
    let from_right = ['-', 'J', '7'].contains(&grid[start.0][start.1+1]);
    match (from_above, from_below, from_left, from_right) {
        (true, true, _, _) => '|',
        (_, _, true, true) => '-',
        (true, _, true, _) => 'J',
        (true, _, _, true) => 'L',
        (_, true, true, _) => '7',
        (_, true, _, true) => 'F',
        _ => '.',
    }
}

fn tile2quads(tile: char) -> ((isize, isize), (isize, isize)) {
    match tile {
        '|' => ((-1,-2),(-1,-2)),
        '-' => ((-1,-1),(-2,-2)),
        'L' => ((-1,-2),(-1,-1)),
        'J' => ((-2,-1),(-1,-1)),
        '7' => ((-1,-1),(-2,-1)),
        'F' => ((-1,-1),(-1,-2)),
        _ => ((-1, -1),(-1, -1)),
    }
}

fn day10a(infile: &str) -> usize {
    let input: Vec<Vec<_>> = infile.lines().map(|l| l.chars().collect()).collect();

    //let start: Vec<Vec<(usize, char)>> = input.iter().map(|l| l.iter().map(|c| c > 0).collect()).collect();
    let start: (usize, usize) = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, ch)| if ch == &'S' { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .next()
        .unwrap();
    let rows = input.len() as isize;
    let cols = input[0].len() as isize;

    println!("start {:?} (would be '{}'", start, reconstruct_start(&input));
    let start_moves = MOVES
        .iter()
        .map(|m| (start.0 as isize + m.0, start.1 as isize + m.1))
        .filter(|m| m.0 >= 0 && m.1 >= 0 && m.0 < rows && m.1 < cols)
        .map(|m| (m.0 as usize, m.1 as usize))
        .filter(|m| next(&input, *m).contains(&start))
        .collect::<Vec<(usize, usize)>>();
    println!("start_moves: {:?}", start_moves);
    let mut queue: Vec<(usize, usize)> = start_moves;
    let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
    distance.insert(start, 0);
    for s in queue.iter() {
        distance.insert(s.clone(), 1);
    }
    while let Some(m) = queue.pop() {
        let dist: usize = *distance.get(&m).unwrap();
        let next = next(&input, m);
        for n in next {
            if ! distance.contains_key(&n) {
                queue.push(n);
                distance.insert(n, dist+1);
            } else {
                let d = distance.get_mut(&n).unwrap();
                if *d > dist+1 {
                    *d = dist+1;
                    queue.push(n);
                }
            }
        }
    }
    let max = distance.values().max().unwrap();
    let cleaned = input.iter().enumerate().map(|(x,l)| l.iter().enumerate().map(|(y,c)| if distance.contains_key(&(x,y)) { *c } else { '.' } ).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    println!("cleaned:");
    for l in cleaned.iter() {
        println!("{}", l.iter().collect::<String>());
    }

    println!("max {}", max);
    *max 
}

fn day10b(infile: &str) -> usize {
    let input: Vec<Vec<char>> = infile.lines().map(|l| l.chars().collect()).collect();

    //let start: Vec<Vec<(usize, char)>> = input.iter().map(|l| l.iter().map(|c| c > 0).collect()).collect();
    // Iterate and find the start position (denominated by 'S' in the input file)
    let start: (usize, usize) = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, ch)| if ch == &'S' { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .next()
        .unwrap();
    let rows = input.len() as isize;
    let cols = input[0].len() as isize;

    println!("start {:?} (would be '{}'", start, reconstruct_start(&input));
    // Initialize with the possible first moves
    let start_moves = MOVES
        .iter()
        .map(|m| (start.0 as isize + m.0, start.1 as isize + m.1))
        .filter(|m| m.0 >= 0 && m.1 >= 0 && m.0 < rows && m.1 < cols)
        .map(|m| (m.0 as usize, m.1 as usize))
        .filter(|m| next(&input, *m).contains(&start))
        .collect::<Vec<(usize, usize)>>();
    println!("start_moves: {:?}", start_moves);
    let mut queue: Vec<(usize, usize)> = start_moves;
    let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
    distance.insert(start, 0);
    for s in queue.iter() {
        distance.insert(s.clone(), 1);
    }
    while let Some(m) = queue.pop() {
        let dist: usize = *distance.get(&m).unwrap();
        let next = next(&input, m);
        for n in next {
            if ! distance.contains_key(&n) {
                queue.push(n);
                distance.insert(n, dist+1);
            } else {
                let d = distance.get_mut(&n).unwrap();
                if *d > dist+1 {
                    *d = dist+1;
                    queue.push(n);
                }
            }
        }
    }
    let max = distance.values().max().unwrap();
    let cleaned = input.iter().enumerate().map(|(x,l)| l.iter().enumerate().map(|(y,c)| if distance.contains_key(&(x,y)) { *c } else { '.' } ).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    println!("cleaned:");
    for l in cleaned.iter() {
        println!("{}", l.iter().collect::<String>());
    }

    println!("max {}", max);
    *max 
}

fn main() {
    println!("day10a sample (should be ...)");
    day10a(include_str!("sample-day10-1.txt"));
    println!("day10a sample (should be ...)");
    day10a(include_str!("sample-day10-2.txt"));
    println!("day10a sample (should be ...)");
    day10a(include_str!("sample-day10-3.txt"));

    println!("day10a input");
    day10a(include_str!("input-day10.txt"));

    println!("day10b sample (should be 2)");
    day10b(&include_str!("sample-day10b-1.txt"));

    println!("day10b input");
    //day10b(&include_str!("input-day10.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09a() {
        let input = include_str!("sample-day09.txt");
        assert_eq!(day09a(&input), 114);
    }
}
