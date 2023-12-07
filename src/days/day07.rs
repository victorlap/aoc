use std::cmp::Ordering;
use std::collections::{HashMap};
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::str::FromStr;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day07.txt").unwrap();
    let parsed: Vec<Hand> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<Hand>) -> u64 {
    let mut mine = input.to_vec();
    mine.sort();
    mine.reverse();

    mine.into_iter().enumerate().fold(0, |acc, (i, hand)| {
        acc + (hand.bid * (i + 1) as u64)
    })
}

fn sol2(input: &Vec<Hand>) -> u64 {
    let mut mine = input.to_vec();
    mine.sort_by(Hand::compare_sol_2);
    mine.reverse();

    mine.into_iter().enumerate().fold(0, |acc, (i, hand)| {
        acc + (hand.bid * (i + 1) as u64)
    })
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => {
                println!("Cannot parse card: {}", s);
                Err(())
            }
        }
    }
}

impl Card {
    fn cmpl_sol_2(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Card::J, Card::J) => Some(Ordering::Equal),
            (Card::J, _) => Some(Ordering::Greater),
            (_, Card::J) => Some(Ordering::Less),
            (_, _) => self.partial_cmp(other),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.compare(other));
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<&str> = s.split_whitespace().collect();

        Ok(Hand {
            cards: p.get(0).unwrap().chars().map(|c| c.to_string().parse().unwrap()).collect(),
            bid: p.get(1).unwrap().parse().unwrap(),
        })
    }
}

impl Hand {
    fn groups(&self) -> HashMap<Card, u8> {
        self.cards.to_vec().into_iter().fold(
            HashMap::new(),
            |mut map, card| {
                *map.entry(card).or_insert(0) += 1;
                map
            })
    }

    fn upgrade(&self) -> Hand {
        let mut best_card = (Card::Two, 0);

        for group in self.groups() {
            if (group.1 > best_card.1 && group.0 != Card::J) || (group.0 > best_card.0 && group.1 == best_card.1 && group.0 != Card::J) {
                best_card = group
            }
        }

        Hand {
            cards: self.cards.iter().map(|c| if c == &Card::J { best_card.0.clone() } else { c.clone() }).collect(),
            bid: self.bid,
        }
    }

    fn get_type(&self) -> Type {
        let groups = self.groups();

        if groups.len() == 1 {
            return Type::FiveOfAKind;
        }

        if groups.len() == 2 {
            if groups.values().any(|&v| v == 4) {
                return Type::FourOfAKind;
            }

            return Type::FullHouse;
        }

        if groups.len() == 3 {
            if groups.values().any(|&v| v == 3) {
                return Type::ThreeOfAKind;
            }

            return Type::TwoPair;
        }

        if groups.len() == 4 {
            return Type::OnePair;
        }

        Type::HighCard
    }

    fn compare(&self, other: &Self) -> Ordering {
        let type_cmp = self.get_type().partial_cmp(&other.get_type());

        if type_cmp.is_some() && type_cmp.unwrap() != Ordering::Equal {
            return type_cmp.unwrap();
        }

        for i in 0..self.cards.len() {
            let card_cmp = self.cards[i].partial_cmp(&other.cards[i]);
            if card_cmp.is_some() && card_cmp.unwrap() != Ordering::Equal {
                return card_cmp.unwrap();
            }
        }

        return Ordering::Equal;
    }

    fn compare_sol_2(&self, other: &Self) -> Ordering {
        let type_cmp = self.upgrade().get_type().partial_cmp(&other.upgrade().get_type());

        if type_cmp.is_some() && type_cmp.unwrap() != Ordering::Equal {
            return type_cmp.unwrap();
        }

        for i in 0..self.cards.len() {
            let card_cmp = self.cards[i].cmpl_sol_2(&other.cards[i]);
            if card_cmp.is_some() && card_cmp.unwrap() != Ordering::Equal {
                return card_cmp.unwrap();
            }
        }

        return Ordering::Equal;
    }
}


#[cfg(test)]
mod tests {
    use crate::day07;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day07::solve();

        assert_eq!(result.0, Solution::U64(246912307));
        assert_eq!(result.1, Solution::U64(246894760));
    }
}
