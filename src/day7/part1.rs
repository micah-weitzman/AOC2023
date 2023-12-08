
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::utils::read_lines;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
  Two = 1,
  Three = 2,
  Four = 3,
  Five = 4,
  Six = 5,
  Seven  = 6,
  Eight = 7,
  Nine = 8,
  Ten = 9,
  Jack = 10,
  Queen = 11,
  King = 12,
  Ace = 13,
}

impl Card {
  fn from_char(c: char) -> Self {
    match c {
      '2' => Card::Two,
      '3' => Card::Three,
      '4' => Card::Four,
      '5' => Card::Five,
      '6' => Card::Six,
      '7' => Card::Seven,
      '8' => Card::Eight,
      '9' => Card::Nine,
      'T' => Card::Ten,
      'J' => Card::Jack,
      'Q' => Card::Queen,
      'K' => Card::King,
      'A' => Card::Ace,
      _ => panic!("Invalid char to Card")
    }
  }
}

type Cards = Vec<Card>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
  HighCard = 1,
  OnePair = 2,
  TwoPair = 3,
  ThreeKind = 4,
  FullHouse = 5,
  FourKind = 6,
  FiveKind = 7,
}

impl Rank {
  fn from_cards(cards: &Cards) -> Self {
    let mut hash_map = cards.iter().fold(
      HashMap::<&Card, u8>::new(),
      |mut hm, card| {
        *hm.entry(card).or_insert(0) += 1;
        hm
      }
    );

    let (top1, top2) = {
      let mut vals: Vec<u8> = hash_map.into_values().collect();
      vals.sort();
      let mut it = vals.iter().rev();
      let t1 = *it.next().unwrap();
      let t2 = *it.next().unwrap_or(&0);
      (t1, t2)
    };

    match (top1, top2) {
      (5, _) => Rank::FiveKind,
      (4, _) => Rank::FourKind,
      (3, 2) => Rank::FullHouse,
      (3, 1) => Rank::ThreeKind,
      (2, 2) => Rank::TwoPair,
      (2, 1) => Rank::OnePair,
      (1, _) => Rank::HighCard,
      _ => panic!("Invalid Hand")
    }
  }
}


#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
  rank: Rank,
  cards: Cards,
  bet: u64,
}

impl Hand {
  fn from_str(line: &String) -> Self {
    let mut it = line.split(" ");
    let cards: Cards = it.next().unwrap().chars().map(Card::from_char).collect();
    let bet = it.next().unwrap().parse::<u64>().unwrap();
    let rank = Rank::from_cards(&cards);
    Hand { rank, cards, bet }
  }
}

fn compare_same_hand(hand1: &[Card], hand2: &[Card]) -> Ordering {
  for (h1, h2) in hand1.iter().zip(hand2.iter()) {
    let cmp_val = h1.cmp(h2);
    if let Ordering::Equal = cmp_val {
      continue;
    }
    return cmp_val;
  }
  panic!("Something went wrong")
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.rank.cmp(&other.rank) {
      Ordering::Equal => compare_same_hand(&self.cards, &other.cards),
      ord => ord
    }
  }
}

pub fn main(filename: &str) {
  let mut hands: Vec<Hand> = read_lines(filename)
                              .iter()
                              .map(Hand::from_str)
                              .collect();
  hands.sort();

  let res: u64 = hands.iter()
                      .enumerate()
                      .fold(0, |acc, (i, h)| acc + ((i as u64 +1) * h.bet));
  println!("{}", res); // 253910319
}