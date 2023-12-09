use std::collections::HashMap;

// GCD helper, Rust doesn't have one?!
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn next_exit<'a>(
    map: &HashMap<&str, (&'a str, &'a str)>,
    steps: &Vec<char>,
    from: &'a str,
    index: usize,
) -> (usize, &'a str) {
    let mut moves = 0;
    let mut current = from;
    let index = index % steps.len();

    for step in steps.iter().cycle().skip(index) {
        moves += 1;

        let cur = map.get(current).unwrap();
        current = match step {
            'L' => cur.0,
            'R' => cur.1,
            _ => "FOO",
        };
        if current.ends_with("Z") {
            return (moves, current);
        }
    }
    (moves, current)
}

fn day08a(infile: &str) -> usize {
    let mut input = infile.lines();
    let steps: Vec<char> = input.next().unwrap().chars().collect();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    // skip the empty line
    input.next();
    for l in input {
        let strs: Vec<_> = l
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| s.len() > 0)
            .collect();
        map.insert(strs[0], (strs[1], strs[2]));
    }
    //    println!("steps: {:?}", steps);
    //    println!("map: {:?}", map);

    let mut current = "AAA";
    let mut moves = 0;
    for step in steps.iter().cycle() {
        moves += 1;
        let mapentry = map.get(current).unwrap();
        let next = match step {
            'L' => mapentry.0,
            'R' => mapentry.1,
            _ => {
                println!("Unknown direction {}", step);
                "FOO"
            }
        };
        //        println!("next {}", next);
        if next == "ZZZ" {
            println!("Got there in {}", moves);
            break;
        }
        current = next;
    }
    moves
}

fn day08b(infile: &str) -> usize {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut input = infile.lines();
    let steps: Vec<char> = input.next().unwrap().chars().collect();

    // skip the empty line
    input.next();
    for l in input {
        let strs: Vec<_> = l
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| s.len() > 0)
            .collect();
        map.insert(strs[0], (strs[1], strs[2]));
    }

    let starts: Vec<&str> = map
        .keys()
        .filter_map(|k| if k.ends_with("A") { Some(*k) } else { None })
        .collect();

    // OUCH! For every starting point, they reach the exit in a certain multiples
    // of steps.len()!!  Not only that, but they enter a loop that repeates on the
    // same frequency. So, double check that's the case (that after one exit is found,
    // there's iteration from that exit to the same exit on that period).
    let loops: Vec<usize> = starts
        .iter()
        .map(|s| {
            let exit = next_exit(&map, &steps, s, 0);
            let next_exit = next_exit(&map, &steps, exit.1, exit.0);
            if exit.1 != next_exit.1 || exit.0 != next_exit.0 {
                println!(
                    "Non-conforming loop: {}-{} distance {}   then {}-{} distance {}",
                    s,
                    exit.1,
                    exit.0,
                    exit.1,
                    next_exit.1,
                    next_exit.0 - exit.0
                );
            }
            next_exit.0
        })
        .collect();

    let mut common = 1;
    for l in loops {
        let gcd = gcd(common, l);
        common = common / gcd * l;
    }

    println!("common is {}", common);
    common
}

fn main() {
    println!("day08a sample (should be 2)");
    day08a(include_str!("sample-day08a.txt"));

    println!("day08a sample 2 (should be 6)");
    day08a(include_str!("sample-day08a2.txt"));

    println!("day08a input");
    day08a(include_str!("input-day08.txt"));

    println!("day08b sample (should be 6)");
    day08b(&include_str!("sample-day08b.txt"));

    println!("day08b input");
    day08b(&include_str!("input-day08.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08a() {
        let input = include_str!("sample-day08a.txt");
        assert_eq!(day08a(&input), 2);
    }

    #[test]
    fn test_day08a2() {
        let input = include_str!("sample-day08a2.txt");
        assert_eq!(day08a(&input), 6);
    }

    #[test]
    fn test_day08b() {
        let input = include_str!("sample-day08b.txt");
        assert_eq!(day08b(&input), 6); // Replace 281 with the expected result
    }
}
