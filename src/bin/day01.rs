fn day01a(infile: &str) {
    let sum: u32 = infile
        //    let input: Vec<&str> = include_str!("sample-day01.txt")
        .lines()
        .map(|b| b.chars().filter_map(|c| c.to_digit(10)).collect())
        .map(|v: Vec<u32>| v.first().unwrap() * 10 + v.last().unwrap())
        .sum();

    println!("sum {:#?}", sum);
}

const REPLACE: &'static [(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn day01b(infile: &str) {
    let input: Vec<_> = infile
        .lines()
        .map(|line| {
            let mut first = 0;
            'front: for idx in 0..=line.len() {
                if let Some(i) = line[idx..].chars().next().unwrap().to_digit(10) {
                    first = i;
                    break;
                }
                for word in REPLACE.iter() {
                    if line[idx..].starts_with(word.0) {
                        first = word.1;
                        break 'front;
                    }
                }
            }
            let mut last = 0;
            'back: for idx in (0..=line.len()).rev() {
                if let Some(i) = line[..idx].chars().rev().next().unwrap().to_digit(10) {
                    last = i;
                    break;
                }
                for word in REPLACE.iter() {
                    if line[..idx].ends_with(word.0) {
                        last = word.1;
                        break 'back;
                    }
                }
            }
            first * 10 + last
        })
        .collect();

    println!("sum {}", input.iter().sum::<u32>());
}

fn main() {
    println!("day01a sample (should be 142)");
    day01a(include_str!("sample-day01a.txt"));
    println!("day01a input");
    day01a(include_str!("input-day01.txt"));
    println!("day01b sample (should be 281)");
    day01b(include_str!("sample-day01b.txt"));
    println!("day01b input");
    day01b(include_str!("input-day01.txt"));
}
