use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
struct Hand(Vec<char>);

const VALUES: &'static [(char, u32)] = &[
    ('2', 1),
    ('3', 2),
    ('4', 3),
    ('5', 4),
    ('6', 5),
    ('7', 6),
    ('8', 7),
    ('9', 8),
    ('T', 9),
    ('J', 10),
    ('Q', 11),
    ('K', 12),
    ('A', 13),
];

fn cardval(c: &char) -> u32 {
    VALUES
        .iter()
        .filter_map(|v| if v.0 == *c { Some(v.1) } else { None })
        .next()
        .unwrap()
}

impl Hand {
    // Method to get an iterator over the elements of the Vec<char> in Hand
    pub fn iter(&self) -> std::slice::Iter<char> {
        self.0.iter()
    }
}

impl FromIterator<char> for Hand {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        Hand(iter.into_iter().collect())
    }
}

// Count the number of each value into a hashmap, so we can just
// take the values, sort them and use that to figure out what kind
// of hand it is with a simple match.
fn value(hand: &Hand) -> u32 {
    let mut cards = HashMap::new();
    for c in hand.iter() {
        if let Some(count) = cards.get(c) {
            cards.insert(*c, count + 1);
        } else {
            cards.insert(*c, 1);
        };
    }
    // Collect values into a vector and sort it
    let mut cards = cards.values().collect::<Vec<&u32>>();
    cards.sort();
    cards.reverse();

    // - Five of a kind, where all five cards have the same label: AAAAA
    // - Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // - Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // - Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // - Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // - One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // - High card, where all cards' labels are distinct: 23456
    let handvalue = match cards[..] {
        [5] => 8,
        [4, 1] => 7,
        [3, 2] => 6,
        [3, 1, 1] => 5,
        [2, 2, 1] => 4,
        [2, 1, 1, 1] => 3,
        [1, 1, 1, 1, 1] => 2,
        _ => 1,
    };
    //    println!("cards: {:#?} value: {}", hand, handvalue);
    handvalue
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if value(self) == value(other) {
            for (a, b) in self.iter().zip(other.iter()) {
                match cardval(a).cmp(&cardval(b)) {
                    Ordering::Equal => continue,
                    non_eq => return non_eq,
                }
            }
            Ordering::Equal
        } else {
            value(self).cmp(&value(other))
        }
    }
}

fn day07a(infile: &str) {
    let input: Vec<(Hand, usize)> = infile
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bet)| {
            (
                hand.chars().collect::<Hand>(),
                bet.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let mut hands: Vec<(Hand, usize)> = input.clone();
    hands.sort();
    let (hands, bets): (Vec<Hand>, Vec<usize>) = hands.into_iter().unzip();
    let winnings: usize = bets.iter().enumerate().map(|(i, b)| b * (i + 1)).sum();

    println!("winnings {}", winnings);
}

fn day07b(infile: &str) {
    let input: Vec<_> = infile.lines().collect();
}

fn main() {
    println!("day07a sample (should be 142)");
    day07a(include_str!("sample-day07.txt"));
    println!("day07a input");
    day07a(include_str!("input-day07.txt"));
    println!("day07b sample (should be 281)");
    day07b(include_str!("sample-day07.txt"));
    println!("day07b input");
    day07b(include_str!("input-day07.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day07a() {
        let input = include_str!("sample-day07.txt");
        assert_eq!(day07a(input), 142); // Replace 142 with the expected result
    }

    #[test]
    fn test_day07b() {
        let input = include_str!("sample-day07.txt");
        assert_eq!(day07b(input), 281); // Replace 281 with the expected result
    }
}
