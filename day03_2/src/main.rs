use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let solution = solution(input);
    println!("The solution is {}", solution);
}

struct PartNumber {
    num: u32,
    /// Indices of all adjacent symbols
    adjacent_symbols: HashSet<usize>,
}

struct Engine {
    numbers: Vec<PartNumber>,
    symbols: HashMap<usize, char>,
}

impl FromStr for Engine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut numbers = Vec::new();
        let mut symbols = HashMap::new();

        let num_cols = s.lines().next().map(|l| l.len()).unwrap_or(0);
        let lines = s.lines().map(str::trim).enumerate().filter(|l| !l.1.is_empty());
        for (row_idx, row) in lines {
            for number_range in NumberRanges::new(row) {
                let row_offset = row_idx * (num_cols + 1);
                let general_range = Range {
                    start: number_range.start + row_offset,
                    end: number_range.end + row_offset,
                };
                let adjacent_symbols = adjacent_symbol_indices(general_range, s);
                let num = u32::from_str(&row[number_range.start..number_range.end])?;
                numbers.push(PartNumber {
                    num,
                    adjacent_symbols,
                });
            }
        }

        for (idx, c) in s.chars().enumerate() {
            if is_symbol(c) {
                symbols.insert(idx, c);
            }
        }

        Ok(Engine {
            numbers,
            symbols,
        })
    }
}

pub struct AdjacentIterator<'a> {
    input: &'a str,
    idx: usize,
    row_len: usize,
    offset: u8,
}

impl<'a> AdjacentIterator<'a> {

    fn idx_to_coord(&self) -> (i32, i32) {
        let row = self.idx / (self.row_len + 1);
        let col = self.idx % (self.row_len + 1);
        (row as i32, col as i32)
    }

    fn coord_to_idx(&self, c: (i32, i32)) -> Option<usize> {
        if c.0 < 0 || c.1 < 0 {
            return None;
        }
        let row = c.0 as usize;
        let col = c.1 as usize;
        Some(row * (self.row_len + 1) + col)
    }

    fn offset_coord_to_idx(&self, offsets: (i32, i32)) -> Option<usize> {
        let mut center = self.idx_to_coord();
        center.0 += offsets.0;
        center.1 += offsets.1;
        self.coord_to_idx(center)
    }

    /// Get the cell value of the adjacent cell at the specified offset.
    /// Offsets go clockwise around the center, starting at the top left.
    fn get_at_offset(&self, offset: u8) -> Option<(usize, char)> {
        let idx = match offset {
            0 => self.offset_coord_to_idx((-1, -1)),
            1 => self.offset_coord_to_idx((0, -1)),
            2 => self.offset_coord_to_idx((1, -1)),
            3 => self.offset_coord_to_idx((1, 0)),
            4 => self.offset_coord_to_idx((1, 1)),
            5 => self.offset_coord_to_idx((0, 1)),
            6 => self.offset_coord_to_idx((-1, 1)),
            7 => self.offset_coord_to_idx((-1, 0)),
            _ => None,
        };
        let input = self.input;
        idx.and_then(move |i| {
            let c = input.chars().skip(i).next()?;
            Some((i, c))
        })
    }
}

impl<'a> Iterator for AdjacentIterator<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < 8 {
            if let Some(c) = self.get_at_offset(self.offset) {
                self.offset += 1;
                return Some(c);
            }
            self.offset += 1;
        }
        None
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.' && c != '\n'
}

struct NumberRanges<'a> {
    begin: usize,
    line: &'a str,
}

impl<'a> NumberRanges<'a> {
    fn new(line: &'a str) -> Self {
        NumberRanges {
            begin: 0,
            line,
        }
    }
}

impl<'a> Iterator for NumberRanges<'a> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {

        // Seek for the next number to start
        for c in self.line.chars().skip(self.begin) {
            if c.is_digit(10) {
                break;
            }
            self.begin += 1;
        }

        // No number found
        if self.begin == self.line.len() {
            return None
        }

        // Seek until the end of the number
        let mut cur = self.begin;
        for c in self.line.chars().skip(self.begin) {
            if !c.is_digit(10) {
                let range = Range {
                    start: self.begin,
                    end: cur,
                };
                self.begin = cur + 1;
                return Some(range);
            }
            cur += 1;
        }

        // No end, the number goes all the way to the end
        let r = Range {
            start: self.begin,
            end: self.line.len(),
        };
        self.begin = self.line.len();
        Some(r)
    }
}

/// Check if any symbol around the range of a number is a symbol
fn adjacent_symbol_indices(number_range: Range<usize>, input: &str) -> HashSet<usize> {
    let mut res = HashSet::new();
    for idx in number_range {
        let it = AdjacentIterator {
            input,
            idx,
            row_len: input.lines().next().map(|l| l.len()).unwrap_or(0),
            offset: 0,
        };
        for (idx, adjacent) in it {
            if is_symbol(adjacent) {
                res.insert(idx);
            }
        }
    }
    res
}

fn solution(input: &str) -> u32 {
    let engine = Engine::from_str(input).expect("Invalid input");
    let mut sum = 0;

    for (cog_idx, _) in engine.symbols.iter().filter(|(_, sym)| **sym == '*') {
        let mut adjacent_numbers = Vec::new();
        for number in &engine.numbers {
            if number.adjacent_symbols.contains(cog_idx) {
                adjacent_numbers.push(number.num);
            }
        }

        if adjacent_numbers.len() == 2 {
            let num: u32 = adjacent_numbers.into_iter().product();
            sum += num;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_case() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
        assert_eq!(solution(input), 467835);
    }
}