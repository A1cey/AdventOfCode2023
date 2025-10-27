pub fn solve() {
    let (part_one, part_two) = solution1::solve();

    dbg!(part_one);
    dbg!(part_two);
}

pub mod solution1 {
    pub fn solve() -> (u32, u32) {
        let data = include_str!("../data/day2.txt");

        let part_one: u32 = data
            .lines()
            .filter_map(part_one)
            .sum();

        let part_two: u32 = data.lines().map(part_two).sum();

        (part_one, part_two)
    }

    fn part_one(line: &str) -> Option<u32> {
        let (game, values) = line.split_once(": ").unwrap();

        let game_id: u32 = game.strip_prefix("Game ").unwrap().parse().unwrap();

        values
            .split("; ")
            .flat_map(|draw| {
                draw.split(", ").map(|color| {
                    let (num_str, col) = color.split_once(" ").unwrap();

                    let num: u8 = num_str.parse().unwrap();

                    match col {
                        "red" => num <= 12,
                        "green" => num <= 13,
                        "blue" => num <= 14,
                        _ => unreachable!(),
                    }
                })
            })
            .all(|valid| valid)
            .then_some(game_id)
    }

    fn part_two(line: &str) -> u32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        line.split_once(": ")
            .unwrap()
            .1
            .split("; ")
            .for_each(|draw| {
                draw.split(", ").for_each(|color| {
                    let (num_str, col) = color.split_once(" ").unwrap();

                    let num = num_str.parse().unwrap();

                    match col {
                        "red" => max_red = max_red.max(num),
                        "green" => max_green = max_green.max(num),
                        "blue" => max_blue = max_blue.max(num),
                        _ => unreachable!(),
                    }
                })
            });

        max_red * max_green * max_blue
    }
}
