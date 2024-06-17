
fn distance(from: &(usize, usize), to: &(usize, usize)) -> usize {
    let f: (isize, isize) = (from.0 as isize, from.1 as isize);
    let t: (isize, isize) = (to.0 as isize, to.1 as isize);
    let ret = (f.0 - t.0).abs() + (f.1 - t.1).abs();
    ret as usize
}

fn day11a(infile: &str) -> usize {
    let input: Vec<Vec<char>> = infile.lines().map(|l| l.chars().collect()).collect();

    let gals: Vec<(usize, usize)> = input.iter().enumerate().flat_map(|(x,l)| l.iter().enumerate().filter_map(|(y,c)| if *c == '#' { Some((x,y)) } else { None }).collect::<Vec<(usize,usize)>>()).collect();

    // Can can easily filter out the coordinates for all galaxies from possible rows/columns to duplicate
    let gal_x: Vec<usize> = gals.iter().map(|(x,_)| *x).collect();
    let gal_y: Vec<usize> = gals.iter().map(|(_,y)| *y).collect();
    let dup_x: Vec<usize> = (0..input.len()).filter(|x| ! gal_x.contains(x)).collect();
    let dup_y: Vec<usize> = (0..input.len()).filter(|y| ! gal_y.contains(y)).collect();

    // Now let's adjust the coordinates for all galaxies accordingly
    let exp_gals: Vec<(usize, usize)> = gals.into_iter().map(|(x,y)| (x + dup_x.iter().filter(|&xx| xx < &x).count(), y+dup_y.iter().filter(|&yy| yy < &y).count())).collect();

    let mut pairs = Vec::new();
    for i in 0..exp_gals.len() {
        for j in (i+1)..exp_gals.len() {
            pairs.push((exp_gals[i], exp_gals[j]));
        }
    }

    let total = pairs.iter().map(|(from, to)| distance(from, to)).sum();
    println!("total: {}", total);
    total 
}

fn main() {
    println!("day11a sample (should be ...)");
    day11a(include_str!("sample-day11.txt"));

    println!("day11a input");
    day11a(include_str!("input-day11.txt"));
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
