use rand::Rng;
use std::fmt::Display;

struct Deck(Vec<Card>);

#[derive(Clone, Copy)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

struct Card {
    suit: Suit,
    value: i32,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::<Card>::with_capacity(52);
        cards.append(&mut Deck::build_suit(Suit::Heart));
        cards.append(&mut Deck::build_suit(Suit::Spade));
        cards.append(&mut Deck::build_suit(Suit::Diamond));
        cards.append(&mut Deck::build_suit(Suit::Club));
        Deck(cards)
    }
    fn build_suit(suit: Suit) -> Vec<Card> {
        let mut cards = Vec::<Card>::with_capacity(13);
        for n in 1..=cards.capacity() {
            cards.push(Card {
                suit,
                value: n as i32,
            })
        }
        cards
    }
    fn shuffle(&mut self, nb_permutations: i32) {
        let mut rng = rand::thread_rng();
        for _ in 0..nb_permutations {
            let first_random_index = rng.gen_range(0..self.0.len());
            let second_random_index = rng.gen_range(0..self.0.len());
            self.0.swap(first_random_index, second_random_index);
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            Self::Heart => String::from("♥️"),
            Self::Spade => String::from("♠️"),
            Self::Club => String::from("♣️"),
            Self::Diamond => String::from("♦️"),
        };
        write!(f, "{}", emoji)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.value {
            1 => String::from("A"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            other_value => other_value.to_string(),
        };
        write!(f, "{}{}", value, self.suit)
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for single_card in self.0.iter() {
            write!(f, "{} ", single_card)?;
        }
        Ok(())
    }
}

impl PartialEq<Card> for Card {
    fn eq(&self, other: &Card) -> bool {
        self.value == other.value && self.suit.to_string() == other.suit.to_string()
    }
}

impl PartialEq<Deck> for Deck {
    fn eq(&self, other: &Deck) -> bool {
        for (i, _) in self.0.iter().enumerate() {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut deck = Deck::new();
    println!("Starting deck:\n{}\n", deck);
    deck.shuffle(5);
    println!("After shuffling deck 5 times:\n{}\n", deck);
    deck.shuffle(100);
    println!("After shuffling deck 100 times:\n{}\n", deck);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_deck_test() {
        let deck = Deck::new();
        assert_eq!(52, deck.0.len());
        let first_card = Card {
            suit: Suit::Heart,
            value: 1,
        };
        let last_card = Card {
            suit: Suit::Club,
            value: 13,
        };
        assert!(first_card == deck.0[0]);
        assert!(last_card == deck.0[deck.0.len() - 1]);
    }

    #[test]
    fn shuffle_test() {
        let ordered_deck = Deck::new();
        let mut unordered_deck = Deck::new();
        unordered_deck.shuffle(100);
        assert!(ordered_deck != unordered_deck);
    }
}
