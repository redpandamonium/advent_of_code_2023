use std::str::FromStr;
use anyhow::anyhow;

#[derive(Clone)]
struct Card {
    idx: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {

        let (first, second) = line.split_once("|")
            .ok_or(anyhow!("Invalid card format. Expected '|'."))?;
        let (card_name, winning_number_str) = first.split_once(":")
            .ok_or(anyhow!("Invalid card format. Expected ':'."))?;
        let (_, card_idx) = card_name.split_once(" ")
            .ok_or(anyhow!("Invalid card format. Missing index."))?;

        let idx = u32::from_str(card_idx.trim())?;
        let mut winning_numbers = Vec::new();
        for winning_number in winning_number_str.split_whitespace() {
            let n = u32::from_str(winning_number)?;
            winning_numbers.push(n);
        }
        let mut numbers = Vec::new();
        for number in second.split_whitespace() {
            let n = u32::from_str(number)?;
            numbers.push(n);
        }

        Ok(Self {
            idx,
            winning_numbers,
            numbers,
        })
    }
}

fn solution(input: &str) -> anyhow::Result<u32> {

    let mut cards = Vec::new();
    for (idx, line) in input.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
        let card = Card::from_str(line)
            .map_err(|e| e.context(format!("Card {}", idx + 1)))?;
        let num_wins = card.numbers.iter()
            .filter(|n| card.winning_numbers.contains(*n))
            .count() as u32;
        cards.push((num_wins, 1));
    }

    for i in 0..cards.len() {
        let (wins, copies) = cards.get(i).unwrap().clone();
        for j in (i + 1)..(i + wins as usize + 1).min(cards.len()) {
            cards.get_mut(j).unwrap().1 += copies;
        }
    }

    Ok(cards.into_iter().map(|(_, copies)| copies).sum())
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");
    let res = solution(input)?;
    println!("The solution is {}", res);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_case() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        assert_eq!(solution(input).unwrap(), 30);
    }
}