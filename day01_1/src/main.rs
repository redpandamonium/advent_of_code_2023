use std::ops::Deref;

fn main() {
    let input = include_str!("input.txt");
    println!("The solution is {}.", trebuchet_value(input));
}

fn trebuchet_value(lines: &str) -> u32 {
    lines.lines().map(calibration_value).map(Option::unwrap).sum()
}

fn calibration_value(line: &str) -> Option<u32> {

    let first = first_digit(line.chars());
    let last = first_digit(line.chars().rev());

    first.zip(last)
        .map(|(a, b)| a * 10 + b)
}

/// Get the first digit in a char iterator as an u32, if it exists.
fn first_digit(mut it: impl Iterator<Item=char>) -> Option<u32> {

    let is_digit = | c: &char | -> bool {
        c.is_digit(10)
    };
    let to_u32 = | c: char | -> Option<u32> {
        c.to_digit(10)
    };

    it.find(is_digit).and_then(to_u32)
}

#[cfg(test)]
mod test {
    use crate::trebuchet_value;

    #[test]
    fn simple_case() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(trebuchet_value(input), 142);
    }
}
