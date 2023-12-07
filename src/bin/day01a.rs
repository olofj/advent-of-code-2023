fn main() {
    let input: u32 = include_str!("input-day01.txt")
        //    let input: Vec<&str> = include_str!("sample-day01.txt")
        .lines()
        .map(|b| b.chars().filter_map(|c| c.to_digit(10)).collect())
        .map(|v: Vec<u32>| v.first().unwrap() * 10 + v.last().unwrap())
        .sum();

    println!("input {:#?}", input);
}
