pub(crate) fn solve(doc: &str) -> Option<u32> {
    doc.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> Option<u32> {
    let digists: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    match digists.len() {
        0 => None,
        1 => Some(10 * digists[0] + digists[0]),
        _ => Some(10 * digists[0] + digists[digists.len() - 1]),
    }
}

#[cfg(test)]
mod tests {
    use crate::solver1::parse_line;

    #[test]
    fn parse_line_tests() {
        assert_eq!(parse_line("1abc2"), Some(12));
        assert_eq!(parse_line("pqr3stu8vwx"), Some(38));
        assert_eq!(parse_line("a1b2c3d4e5f"), Some(15));
        assert_eq!(parse_line("treb7uchet"), Some(77));
    }
}
