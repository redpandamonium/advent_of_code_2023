use std::str::FromStr;
use anyhow::anyhow;

struct Quantities {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Quantities {
    /// Returns true iff all quantities inside self are within the limits set by other.
    fn fits_inside(&self, other: &Quantities) -> bool {
        if self.reds > other.reds {
            false
        }
        else if self.greens > other.greens {
            false
        }
        else if self.blues > other.blues {
            false
        }
        else {
            true
        }
    }
}

struct Game {
    id: i32,
    rounds: Vec<Quantities>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (game, content) = line.split_once(":").ok_or(anyhow!("Missing ':'"))?;
        let (_, id) = game.split_once(" ").ok_or(anyhow!("Missing game id"))?;
        let game_id = i32::from_str(id)?;
        let rounds = content.split(";");
        let mut rounds_vec = Vec::new();

        for round in rounds {

            let mut reds = 0;
            let mut greens = 0;
            let mut blues = 0;

            let quantities = round.split(",");
            for quantity in quantities {
                let (num, color) = quantity.trim().split_once(" ").ok_or(anyhow!("Missing color"))?;
                let num = u32::from_str(num)?;
                match color {
                    "red" => reds += num,
                    "green" => greens += num,
                    "blue" => blues += num,
                    _ => return Err(anyhow!("Unknown color {}", color)),
                }
            }

            rounds_vec.push(Quantities {
                reds,
                greens,
                blues,
            })
        }
        Ok(Game {
            id: game_id,
            rounds: rounds_vec,
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    let available = Quantities {
        reds: 12,
        greens: 13,
        blues: 14,
    };
    let solution = solution(input, available);
    println!("The solution is {}", solution);
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Game>> {

    fn effectively_empty(t: &(usize, &str)) -> bool {
        !t.1.is_empty()
    }

    let mut res = Vec::new();
    let lines = input.lines()
        .map(str::trim)
        .enumerate()
        .filter(effectively_empty);

    for (line_idx, line) in lines {
        res.push(Game::from_str(line)
            .map_err(|err| err.context(format!("Invalid game on line {}", line_idx + 1)))?);
    }
    Ok(res)
}

fn solution(input: &str, available: Quantities) -> i32 {
    let games = parse_input(input).expect("Invalid input");
    let mut sum = 0;
    for game in games {
        if is_game_possible(&game, &available) {
            sum += game.id;
        }
    }
    sum
}

fn is_game_possible(game: &Game, quantities: &Quantities) -> bool {
    for round in &game.rounds {
        if !round.fits_inside(quantities) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_case() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        let available = Quantities {
            reds: 12,
            greens: 13,
            blues: 14,
        };
        assert_eq!(solution(input, available), 8);
    }
}