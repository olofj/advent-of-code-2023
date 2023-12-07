fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    println!("input {:?}", input);

    let races: Vec<(usize, usize)> = input[0]
        .iter()
        .zip(input[1].iter())
        .map(|(s, d)| (*s, *d))
        .collect();

    println!("races: {:?}", races);

    let wins: Vec<_> = races.iter().map(|r| {
        let time = r.0;
        let distance = r.1;
        let mut wins = 0;
        for hold in 0..=time {
            let speed = hold;
            let mydistance = (time - hold) * speed;
            if mydistance > distance {
                wins += 1;
            }
        }
        wins
    }).collect();
    println!("Wins: {:?}", wins);
    println!("Ways to win: {}", wins.iter().product::<usize>());
}
