pub fn solve() {
    let (part_one, part_two) = solution1::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 28750);
    assert_eq!(part_two, 10212704);
}

pub mod solution1 {
    use std::collections::HashSet;

    pub fn solve() -> (u32, u32) {
        let data = include_str!("../data/day4.txt");

        let part_one = data.lines().map(part_one).sum();

        let part_two = part_two(data);

        (part_one, part_two)
    }

    fn part_one(card: &str) -> u32 {
        2u32.pow(num_of_hits(card).saturating_sub(1) as u32)
    }

    fn num_of_hits(card: &str) -> usize {
        let (_, nums) = card.split_once(": ").unwrap();
        let (winning_str, pulled_str) = nums.split_once(" | ").unwrap();

        let winning = setify(winning_str);
        let pulled = setify(pulled_str);

        winning.intersection(&pulled).count()
    }

    fn setify(nums: &str) -> HashSet<&str> {
        nums.split_whitespace()
            .fold(HashSet::new(), |mut acc, num| {
                acc.insert(num);
                acc
            })
    }

    fn part_two(data: &str) -> u32 {
        let hits_per_card = data.lines().map(num_of_hits).collect::<Vec<_>>();

        let mut additional_cards_per_card = vec![0; hits_per_card.len()];

        let mut total_cards = hits_per_card.len();

        for (card, &hits) in hits_per_card.iter().enumerate().rev() {
            let mut additional_cards = 0;

            for hit in 1..=hits {
                additional_cards += additional_cards_per_card[card + hit] + 1; // plus 1 for the first additional card
            }

            total_cards += additional_cards;
            additional_cards_per_card[card] = additional_cards;
        }

        total_cards as u32
    }
}
