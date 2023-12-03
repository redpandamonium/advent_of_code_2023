use std::ops::Range;
use std::str::FromStr;
use anyhow::anyhow;

fn main() {
    let input = include_str!("input.txt");
    let solution = solution(input);
    println!("The solution is {}", solution);
}

struct TextMatrix(Vec<Vec<char>>);

impl TextMatrix {

    /// Get the character in a specific cell
    fn char_at(&self, col: i32, row: i32) -> Option<&char> {
        if col < 0 || row < 0 {
            return None;
        }
        self.0.get(row as usize).and_then(|r| r.get(col as usize))
    }

    fn range_as_str(&self, row: i32, range: Range<i32>) -> Option<String> {
        let mem = self.0.get(row as usize)
            .map(|r: &Vec<char>| &r[range.start as usize..range.end as usize])?;
        Some(String::from_iter(mem.iter()))
    }

    /// Get an iterator over all the adjacent cells that are in bounds
    fn adjacent(&self, col: i32, row: i32) -> TextMatrixAdjacentIterator {
        TextMatrixAdjacentIterator {
            mat: self,
            col,
            row,
            offset: 0,
        }
    }

    fn num_cols(&self) -> i32 {
        self.0.get(0).map(|v| v.len()).unwrap_or(0) as i32
    }

    fn num_rows(&self) -> i32 {
        self.0.len() as i32
    }
}

pub struct TextMatrixAdjacentIterator<'a> {
    mat: &'a TextMatrix,
    col: i32,
    row: i32,
    offset: u8,
}

impl<'a> TextMatrixAdjacentIterator<'a> {
    /// Get the cell value of the adjacent cell at the specified offset.
    /// Offsets go clockwise around the center, starting at the top left.
    fn get_at_offset(&self, offset: u8) -> Option<&'a char> {
        match offset {
            0 => self.mat.char_at(self.col - 1, self.row - 1),
            1 => self.mat.char_at(self.col, self.row - 1),
            2 => self.mat.char_at(self.col + 1, self.row - 1),
            3 => self.mat.char_at(self.col + 1, self.row),
            4 => self.mat.char_at(self.col + 1, self.row + 1),
            5 => self.mat.char_at(self.col, self.row + 1),
            6 => self.mat.char_at(self.col - 1, self.row + 1),
            7 => self.mat.char_at(self.col - 1, self.row),
            _ => None,
        }
    }
}

impl<'a> Iterator for TextMatrixAdjacentIterator<'a> {
    type Item = &'a char;

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

impl FromStr for TextMatrix {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(str::trim).filter(|l| !l.is_empty());
        let mut mat: Vec<Vec<char>> = Vec::new();
        for line in lines {
            mat.push(line.chars().collect());
        }
        let line_len = mat.get(0).map(|v| v.len());
        for line in &mat {
            if line.len() != line_len.unwrap() {
                return Err(anyhow!("Inconsistent line lengths."));
            }
        }
        Ok(Self {
            0: mat,
        })
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

struct NumberRanges<'a> {
    begin: i32,
    row: i32,
    mat: &'a TextMatrix,
}

impl<'a> Iterator for NumberRanges<'a> {
    type Item = Range<i32>;

    fn next(&mut self) -> Option<Self::Item> {

        // Seek for the next number to start
        while let Some(ch) = self.mat.char_at(self.begin, self.row) {
            if ch.is_digit(10) {
                break;
            }
            self.begin += 1;
        }

        // No number found
        if self.begin == self.mat.num_cols() {
            return None
        }

        // Seek until the end of the number
        let mut cur = self.begin + 1;
        while let Some(ch) = self.mat.char_at(cur, self.row) {
            if !ch.is_digit(10) {
                let range = Range {
                    start: self.begin,
                    end: cur,
                };
                self.begin = cur;
                return Some(range);
            }
            cur += 1;
        }

        // No end, the number goes all the way to the end
        let r = Range {
            start: self.begin,
            end: self.mat.num_cols(),
        };
        self.begin = cur;
        Some(r)
    }
}

/// Check if any symbol around the range of a number is a symbol
fn has_symbol_adjacent(row: i32, number_range: Range<i32>, mat: &TextMatrix) -> bool {
    for col in number_range {
        for adjacent in mat.adjacent(col, row) {
            if is_symbol(*adjacent) {
                return true;
            }
        }
    }
    false
}

fn solution(input: &str) -> u32 {
    let mat = TextMatrix::from_str(input).expect("Invalid input");
    let mut sum = 0;
    for row in 0..mat.num_rows() {
        println!("Row {}", row);
        let it = NumberRanges {
            begin: 0,
            row,
            mat: &mat,
        };
        for number_range in it {
            if has_symbol_adjacent(row, number_range.clone(), &mat) {
                let s = mat.range_as_str(row, number_range).unwrap();
                let num = u32::from_str(s.as_str()).unwrap();
                sum += num;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_case() {
        let input = r#"
            467..114..
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
        assert_eq!(solution(input), 4361);
    }

    #[test]
    fn problematic_case_01() {
        let input = r#"*920"#;
        assert_eq!(solution(input), 920);
    }
}