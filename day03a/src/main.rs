const MOVES: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn moves(x: usize, y: usize, xmax: usize, ymax: usize) -> Vec<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    let xmax = xmax as isize;
    let ymax = ymax as isize;
    MOVES
        .iter()
        .map(|&(dx, dy)| (dx + x, dy + y))
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < xmax && y < ymax)
        .map(|(x,y)| (x as usize, y as usize))
        .collect::<Vec<_>>()
}

fn printgrid(lc: &Vec<Vec<char>>) {
    for l in lc.iter() {
        for c in l.iter() {
            print!("{}", c);
        }
        println!("");
    }
}

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let nlines = grid.len();
    let nchars = grid[0].len();

    //println!("grid {:?}", grid);
    println!("grid");
    printgrid(&grid);

    let symbols = grid
        .iter()
        .enumerate()
        .flat_map(|(lc, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(cc, &c)| {
                    if !c.is_digit(10) && c != '.' {
                        Some((lc, cc))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("symbols {:#?}", symbols);

    let mut mask = vec![vec![false; nchars]; nlines];
    for (sl,sc) in symbols {
        let mut moves = moves(sl,sc,nlines,nchars);
        while let Some((l,c)) = moves.pop() {
            if grid[l][c].is_digit(10) {
                mask[l][c] = true;
                if c < nchars-1 && ! mask[l][c+1] {
                    moves.push((l, c+1));
                }
                if c > 0 && !mask[l][c-1] {
                    moves.push((l, c-1));
                }
            }
        }
    }
    println!("mask");
    printgrid(&mask.iter().map(|l| l.iter().map(|&b| if b { 'X' } else { '.' } ).collect()).collect());

    let masked: String = grid.iter().zip(mask.iter()).map(|(l,&ref lm)| l.iter().zip(lm.iter()).map(|(&c,&mc)| if mc { c} else {' '}).collect::<String>() + " ").collect::<String>();
    let vals: Vec<usize> = masked.split(" ").filter_map(|s| s.parse::<usize>().ok()).collect();
    println!("vals {:#?}", vals);
    println!("sum {}", vals.iter().sum::<usize>());
}
