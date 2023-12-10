fn day09a(infile: &str) -> isize {
    let input: Vec<Vec<_>> = infile
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|w| w.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    let results: Vec<isize> = input
        .iter()
        .map(|l| {
            let mut history: Vec<Vec<_>> = vec![l.clone()];
            let mut last = l.clone();
            while last.iter().any(|a| *a != 0) {
                let next: Vec<_> = last.windows(2).map(|v| v[1] - v[0]).collect();
                last = next.clone();
                history.push(next);
            }
            let mut acc = 0;
            let lasts: Vec<_> = history.iter().map(|r| *r.last().unwrap()).collect();
            let n: Vec<_> = lasts
                .iter()
                .rev()
                .map(|e| {
                    acc += *e;
                    acc
                })
                .collect();
            *n.last().unwrap()
        })
        .collect();
    let sum: isize = results.iter().sum();
    println!("sum {}", sum);

    sum
}

fn day09b(infile: &str) -> isize {
    let input: Vec<Vec<_>> = infile
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|w| w.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    let results: Vec<isize> = input
        .iter()
        .map(|l| {
            let mut history: Vec<Vec<_>> = vec![l.clone()];
            let mut last = l.clone();
            while last.iter().any(|a| *a != 0) {
                let next: Vec<_> = last.windows(2).map(|v| v[1] - v[0]).collect();
                last = next.clone();
                history.push(next);
            }
            let mut acc = 0;
            let firsts: Vec<_> = history.iter().map(|r| *r.first().unwrap()).collect();
            let n: Vec<_> = firsts
                .iter()
                .rev()
                .map(|e| {
                    acc = *e - acc;
                    acc
                })
                .collect();
            *n.last().unwrap()
        })
        .collect();
    let sum: isize = results.iter().sum();
    println!("sum {}", sum);

    sum
}

fn main() {
    println!("day09a sample (should be 114)");
    day09a(include_str!("sample-day09.txt"));

    println!("day09a input");
    day09a(include_str!("input-day09.txt"));

    println!("day09b sample (should be 2)");
    day09b(&include_str!("sample-day09.txt"));

    println!("day09b input");
    day09b(&include_str!("input-day09.txt"));
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
