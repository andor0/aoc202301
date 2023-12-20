pub(crate) fn solve(doc: &str) -> Option<u32> {
    doc
        .lines()
        .map(parse_line)
        .sum()
}

fn parse_line(line: &str) -> Option<u32> {
    if let (Some(first_digit), Some(last_digit)) = (find_first_digit(line), find_last_digit(line)) {
        Some(10 * first_digit + last_digit)
    } else {
        None
    }
}

fn find_first_digit(line: &str) -> Option<u32> {
    const LETERS_TO_DIGITS: [(&str, u32); 19] = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut result = None;
    let mut min_index = Some(line.len());

    for (leter, digit) in LETERS_TO_DIGITS.iter() {
        let index = line.find(leter);
        if index.is_some() && index < min_index {
            min_index = index;
            result = Some(*digit);
        }
    }

    result
}

fn find_last_digit(line: &str) -> Option<u32> {
    const LETERS_TO_DIGITS_REVERSE: [(&str, u32); 19] = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ];

    let buf: String = line.chars().rev().collect();
    let mut result = None;
    let mut min_index = Some(line.len());

    for (leter, digit) in LETERS_TO_DIGITS_REVERSE.iter() {
        let index = buf.find(leter);
        if index.is_some() && index < min_index {
            min_index = index;
            result = Some(*digit);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::solver2::*;

    #[test]
    fn parse_line_tests() {
        assert_eq!(parse_line("two1nine"), Some(29));
        assert_eq!(parse_line("eightwothree"), Some(83));
        assert_eq!(parse_line("abcone2threexyz"), Some(13));
        assert_eq!(parse_line("xtwone3four"), Some(24));
        assert_eq!(parse_line("4nineeightseven2"), Some(42));
        assert_eq!(parse_line("zoneight234"), Some(14));
        assert_eq!(parse_line("7pqrstsixteen"), Some(76));
    }
}
