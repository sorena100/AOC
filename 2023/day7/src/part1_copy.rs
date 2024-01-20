use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

fn card_comp(a: &Card, b: &Card) -> std::cmp::Ordering {
    match (a, b) {
        (Card::Ace, _) => std::cmp::Ordering::Greater,
        (_, Card::Ace) => std::cmp::Ordering::Less,
        (Card::King, _) => std::cmp::Ordering::Greater,
        (_, Card::King) => std::cmp::Ordering::Less,
        (Card::Queen, _) => std::cmp::Ordering::Greater,
        (_, Card::Queen) => std::cmp::Ordering::Less,
        (Card::Jack, _) => std::cmp::Ordering::Greater,
        (_, Card::Jack) => std::cmp::Ordering::Less,
        (Card::Number(a), Card::Number(b)) => a.cmp(b),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn hand_type_comp(a: &HandType, b: &HandType) -> std::cmp::Ordering {
    if a == b {
        return std::cmp::Ordering::Equal;
    }
    return match (a, b) {
        (HandType::FiveOfAKind, _) => std::cmp::Ordering::Greater,
        (_, HandType::FiveOfAKind) => std::cmp::Ordering::Less,
        (HandType::FourOfAKind, _) => std::cmp::Ordering::Greater,
        (_, HandType::FourOfAKind) => std::cmp::Ordering::Less,
        (HandType::FullHouse, _) => std::cmp::Ordering::Greater,
        (_, HandType::FullHouse) => std::cmp::Ordering::Less,
        (HandType::ThreeOfAKind, _) => std::cmp::Ordering::Greater,
        (_, HandType::ThreeOfAKind) => std::cmp::Ordering::Less,
        (HandType::TwoPairs, _) => std::cmp::Ordering::Greater,
        (_, HandType::TwoPairs) => std::cmp::Ordering::Less,
        (HandType::OnePair, _) => std::cmp::Ordering::Greater,
        (_, HandType::OnePair) => std::cmp::Ordering::Less,
        (HandType::HighCard, _) => std::cmp::Ordering::Greater,
        (_, HandType::HighCard) => std::cmp::Ordering::Less,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort_by(|a, b| card_comp(a, b));
        let deduced = sorted_cards.iter().dedup_with_count();
        let mut counts = deduced.map(|(count, _)| count).collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPairs,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn compare(&self, other: &Hand) -> std::cmp::Ordering {
        let a = self.get_hand_type();
        let b = other.get_hand_type();
        let comp = hand_type_comp(&a, &b);
        if comp != std::cmp::Ordering::Equal {
            return comp;
        }
        return self.compare_by_high_card(other);
    }

    fn compare_by_high_card(&self, other: &Hand) -> std::cmp::Ordering {
        let a = self.cards.clone();
        let b = other.cards.clone();
        for i in 0..5 {
            let a_card = a.get(i).unwrap();
            let b_card = b.get(i).unwrap();
            if &a_card == &b_card {
                continue;
            }
            let comp = card_comp(&a_card, &b_card);
            if comp != std::cmp::Ordering::Equal {
                return comp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}

pub fn main() {
    let input = r"src\input\input.txt";
    let contents = std::fs::read_to_string(input).unwrap();
    let result = process(&contents);
    println!("Part 1 result: {}", result);
}

fn process(content: &str) -> usize {
    let mut lines = content.lines();
    let mut hands = lines.map(parse_hand).collect::<Vec<_>>();
    hands.sort_by(|a, b| a.compare(b));
    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid * (i + 1);
    }
    result
}

fn parse_card(c: char) -> Card {
    match c {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => Card::Jack,
        'T' => Card::Number(10),
        _ => Card::Number(c.to_digit(10).unwrap() as u8),
    }
}

fn parse_hand(line: &str) -> Hand {
    let mut info = line.split_whitespace();
    let cards = info
        .next()
        .map(|s| s.chars().map(parse_card).collect::<Vec<_>>())
        .unwrap();
    let bid = info.next().unwrap().parse::<usize>().unwrap();
    Hand {
        cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
        bid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process(input);
        assert_eq!(result, 6440);
    }
}
