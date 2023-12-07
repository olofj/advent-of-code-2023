fn main() {
    let input: Vec<_> = include_str!("input-day06.txt")
        .lines()
        .map(|l| {
            l.split_once(":")
                .unwrap()
                .1
                .chars()
                .filter(|c| c != &' ')
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    println!("input {:?}", input);

    let time = input[0];
    let distance = input[1];

    let mut wins = 0;
    for hold in 0..=time {
        let mydistance = hold * (time - hold);
        if mydistance > distance {
            wins += 1;
        }
    }
    println!("wins {}", wins);
}
