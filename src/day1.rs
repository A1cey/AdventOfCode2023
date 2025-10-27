pub fn solve() {
    let (part_one, part_two) = replace_number_words_optimized::solve();

    dbg!(part_one);
    dbg!(part_two);
}

pub mod reverse_regex {
    use regex::Regex;

    pub fn solve() -> (u32, u32) {
        let data = include_str!("../data/day1.txt");

        let re_1 = Regex::new(r"\d").unwrap();
        let re_2 = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let re_2_rev = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

        let part_one = data
            .lines()
            .map(|line| line_to_number(line, &re_1, &re_1))
            .sum::<u32>();

        let part_two = data
            .lines()
            .map(|line| line_to_number(line, &re_2, &re_2_rev))
            .sum::<u32>();

        (part_one, part_two)
    }

    fn line_to_number(line: &str, re: &Regex, re_rev: &Regex) -> u32 {
        let line_rev = line.chars().rev().collect::<String>();

        let first = re.find(line).unwrap();
        let last = re_rev.find(line_rev.as_str()).unwrap();

        let first_digit = str_to_num(first.as_str());
        let last_digit = str_to_num(last.as_str());

        first_digit * 10 + last_digit
    }

    fn str_to_num(s: &str) -> u32 {
        match s {
            "one" | "eno" => 1,
            "two" | "owt" => 2,
            "three" | "eerht" => 3,
            "four" | "ruof" => 4,
            "five" | "evif" => 5,
            "six" | "xis" => 6,
            "seven" | "neves" => 7,
            "eight" | "thgie" => 8,
            "nine" | "enin" => 9,
            c => c.parse().unwrap(),
        }
    }
}

pub mod reverse_regex_optimized {
    use regex::Regex;

    pub fn solve() -> (u32, u32) {
        let data = include_str!("../data/day1.txt");

        let re_2 = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let re_2_rev = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

        let part_one = data.lines().map(line_to_number_part_one).sum::<u32>();

        let part_two = data
            .lines()
            .zip(data.chars().rev().collect::<String>().lines())
            .map(|(line, line_rev)| line_to_number(line, line_rev, &re_2, &re_2_rev))
            .sum::<u32>();

        (part_one, part_two)
    }

    fn line_to_number_part_one(line: &str) -> u32 {
        let first = line.find(char::is_numeric).unwrap();
        let last = line.rfind(char::is_numeric).unwrap();

        let first_digit: u32 = line.chars().nth(first).unwrap().to_digit(10).unwrap();
        let last_digit: u32 = line.chars().nth(last).unwrap().to_digit(10).unwrap();

        first_digit * 10 + last_digit
    }

    fn line_to_number(line: &str, line_rev: &str, re: &Regex, re_rev: &Regex) -> u32 {
        let first = re.find(line).unwrap();
        let last = re_rev.find(line_rev).unwrap();

        let first_digit = str_to_num(first.as_str());
        let last_digit = str_to_num(last.as_str());

        first_digit * 10 + last_digit
    }

    fn str_to_num(s: &str) -> u32 {
        match s {
            "one" | "eno" => 1,
            "two" | "owt" => 2,
            "three" | "eerht" => 3,
            "four" | "ruof" => 4,
            "five" | "evif" => 5,
            "six" | "xis" => 6,
            "seven" | "neves" => 7,
            "eight" | "thgie" => 8,
            "nine" | "enin" => 9,
            c => c.parse().unwrap(),
        }
    }
}

pub mod replace_number_words {

    pub fn solve() -> (u32, u32) {
        let data = include_str!("../data/day1.txt");

        let part_one = data.lines().map(line_to_number).sum::<u32>();

        let data = data
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");

        let part_two = data.lines().map(line_to_number).sum::<u32>();

        (part_one, part_two)
    }

    fn line_to_number(line: &str) -> u32 {
        let first = line.find(char::is_numeric).unwrap();
        let last = line.rfind(char::is_numeric).unwrap();

        let first_digit: u32 = line.chars().nth(first).unwrap().to_digit(10).unwrap();
        let last_digit: u32 = line.chars().nth(last).unwrap().to_digit(10).unwrap();

        first_digit * 10 + last_digit
    }
}

pub mod replace_number_words_optimized {

    pub fn solve() -> (u32, u32) {
        let data = include_str!("../data/day1.txt");

        let part_one = data.lines().map(line_to_number).sum::<u32>();

        let data = data
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");

        let part_two = data.lines().map(line_to_number).sum::<u32>();

        (part_one, part_two)
    }

    fn line_to_number(line: &str) -> u32 {
        let bytes = line.as_bytes();

        let first = bytes.iter().find(|b| b.is_ascii_digit()).unwrap();

        let last = bytes.iter().rfind(|b| b.is_ascii_digit()).unwrap();

        let first_digit = (first - b'0') as u32;
        let last_digit = (last - b'0') as u32;

        10 * first_digit + last_digit
    }
}
