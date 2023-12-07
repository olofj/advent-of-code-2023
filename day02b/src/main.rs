use std::cmp::max;

fn main() {
    let limit: (usize, usize, usize) = (12, 13, 14);
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split_once(": ").unwrap().1)
        .map(|l| {
            l.split("; ")
                .map(|g| {
                    g.split(", ")
                        .map(|group| {
                            let (n, c) = group.split_once(" ").unwrap();
                            let n = n.parse::<usize>().unwrap();
                            match c {
                                "red" => (n, 0, 0),
                                "green" => (0, n, 0),
                                "blue" => (0, 0, n),
                                _ => (0, 0, 0),
                            }
                        })
                        .fold((0, 0, 0), |(aa, ab, ac), (a, b, c)| {
                            (max(aa, a), max(ab, b), max(ac, c))
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("input {:#?}", input);
    let count: usize = input
        .into_iter()
        .enumerate()
        .map(|(a, b)| (a + 1, b))
        .filter(|(_, v)| {
            v.iter()
                .all(|(a, b, c)| (*a <= limit.0 && *b <= limit.1 && *c <= limit.2))
        })
        .map(|(a, _)| a)
        .sum();

    println!("count {:#?}", count);
}
