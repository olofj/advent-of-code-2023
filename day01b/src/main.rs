fn main() {
    let replace = vec![
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
    let input: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut first = 0;
            'front: for idx in 0..=line.len() {
                if let Some(i) = line[idx..].chars().next().unwrap().to_digit(10) {
                    first = i;
                    break;
                }
                for word in replace.iter() {
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
                for word in replace.iter() {
                    if line[..idx].ends_with(word.0) {
                        last = word.1;
                        break 'back;
                    }
                }
            }
            println!("{} => {} {}", line, first, last);
            first * 10 + last
        })
        .collect();

    println!("sum {}", input.iter().sum::<u32>());
}
