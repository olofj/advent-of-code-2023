use std::collections::HashMap;

fn day08a(infile: &str) {
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
}

fn main() {
    println!("day08a sample (should be 2)");
    day08a(include_str!("sample-day08a.txt"));

    println!("day08a sample 2 (should be 6)");
    day08a(include_str!("sample-day08a2.txt"));

    println!("day08a input");
    day08a(include_str!("input-day08.txt"));
}
