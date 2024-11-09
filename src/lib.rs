use nom::{
    IResult,
    branch::alt,
    bytes::complete::{
        tag,
    },
    character::complete::char,
    combinator::value,
    multi::many1,
};

pub fn parse_num_str(input: &str) -> IResult<&str, u8> {
    alt((
        value(0, char('0')),
        value(1, char('1')),
        value(2, char('2')),
        value(3, char('3')),
        value(4, char('4')),
        value(5, char('5')),
        value(6, char('6')),
        value(7, char('7')),
        value(8, char('8')),
        value(9, char('9')),
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

pub fn parse_line(input: &str) -> u8 {

    let mut v: Vec<u8> = Vec::new();
    let mut line = &input[0..];

    while line.len() > 0 {
        let nums = many1(parse_num_str)(line);
        match nums {
            Ok((r, mut ns)) => {
                v.append(&mut ns);
                line = r;
            },
            Err(_) => line = &line[1..],
        }
    }

    10 * v.first().unwrap() + v.last().unwrap()
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_str_one() {
        let exact = "one";
        let extra = "one123";
        let bad = "abc";

        assert_eq!(parse_num_str(exact), Ok(("", 1)));

        assert_eq!(parse_num_str(extra), Ok(("123", 1)));

        assert!(parse_num_str(bad).is_err());

    }

    #[test]
    fn num_str_all_good() {
        let good = [
            ("onetrd6", 1, "trd6"),
            ("twothree", 2, "three"),
            ("three45six", 3, "45six"),
            ("four908x", 4, "908x"),
            ("five", 5, ""),
            ("sixsix8", 6, "six8"),
            ("sevennn0*^", 7, "nn0*^"),
            ("eight", 8, ""),
            ("nineninenine", 9, "ninenine"),
        ];

        for (s, n, r) in good {
            assert_eq!(parse_num_str(s), Ok((r, n)));
        }
    }

    #[test]
    fn num_str_all_bad() {

        let bad = [
            "jahsskdah",
            "onnineigh"
        ];

        for s in bad {
            assert!(parse_num_str(s).is_err());
        }
    }

    #[test]
    fn num_str_all_wrong() {

        let wrong = [
            ("one", 2, ""),
            ("nine", 8, "")
        ];

        for (s, n, r) in wrong {
            assert_ne!(parse_num_str(s), Ok((r, n)));
        }
    }

    #[test]
    fn num_dig_all_good() {
        let good = [
            ("08qdu", 0, "8qdu"),
            ("1nekn7", 1, "nekn7"),
            ("2", 2, ""),
            ("3 fo*", 3, " fo*"),
            ("4Agfo", 4, "Agfo"),
            ("5", 5, ""),
            ("63oi", 6, "3oi"),
            ("7", 7, ""),
            ("8q%$", 8, "q%$"),
            ("9yhs", 9, "yhs"),
        ];

        for (s, n, r) in good {
            assert_eq!(parse_num_str(s), Ok((r, n)));
        }
    }

    #[test]
    fn full_line_check() {

        let mut line = "two1nine";
        assert_eq!(parse_line(line), 29);

        line = "abcone2threexyz";
        assert_eq!(parse_line(line), 13);

        line = "xtwone3four";
        assert_eq!(parse_line(line), 24);
    
    }
}