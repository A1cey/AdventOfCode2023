use std::fs;

use regex::Regex;

pub fn solve() {
    let data = fs::read_to_string("data/day1.txt").unwrap();

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

    dbg!(part_one);
    dbg!(part_two);
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
