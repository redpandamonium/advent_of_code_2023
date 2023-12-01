use itertools::Itertools;

static DIGITS: [&'static str; 18] = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn main() {
    let input = include_str!("input.txt");
    println!("The solution is {}.", trebuchet_value(input));
}

fn trebuchet_value(lines: &str) -> u32 {
    lines.lines()
        .filter(|l| !l.trim().is_empty())
        .map(calibration_value)
        .map(Option::unwrap)
        .sum()
}

fn calibration_value(line: &str) -> Option<u32> {

    // Map the index into the DIGITS array to the numerical digit value
    let from_regex_index = |mut i: usize| -> u32 {
        if i >= 9 {
            i -= 9;
        }
        (i  + 1) as u32
    };

    // Find all occurrences of the pattern.
    // Returns an iterable of (str_idx, pattern_idx)
    let find_pattern = |(pattern_idx, pattern)| {
        line.match_indices(pattern)
            .map(move |(i, _)| (i, pattern_idx))
    };

    let mut it = DIGITS.iter().enumerate()
        // Find all indices where this digit string occurs
        .map(find_pattern)
        .flatten()
        // Sort the matches by ascending index in the line string
        .sorted_by_key(|(i, _)| *i)
        // Discard the line index and map the pattern index to the digit
        .map(|(_, s)| from_regex_index(s));

    let first = it.next();
    let last = it.last().or(first);
    first.zip(last)
        .map(|(a, b)| a * 10 + b)
}

#[cfg(test)]
mod test {
    use crate::trebuchet_value;

    #[test]
    fn simple_case() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;
        assert_eq!(trebuchet_value(input), 281);
    }

    #[test]
    fn problematic_line_01() {
        let input = "pxvmbjprllmbfpzjxsvhc5";
        assert_eq!(trebuchet_value(input), 55);
    }
}