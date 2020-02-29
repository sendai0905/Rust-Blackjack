extern crate rand;

use rand::Rng;
use std::io;

enum Mark {
    Diamond,
    Heart,
    Spade,
    Clover
}

impl Mark {
    fn show(&self) -> {

    }
}

struct Card {
    mark: Mark,
    number: u8,
}

impl Card {
    fn new(mark: Mark, number: &str) -> Card {
        Card {
            mark: mark,
            number: to_number(number),
        }
    }
}

fn generate_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for i in 1..52 {
        if i <= 13 {
            deck.push(Card::new(Mark::Diamond, to_str(&i)));
        } else if i <= 26 {
            deck.push(Card::new(Mark::Heart, to_str(&(i - 13))));
        } else if i <= 39 {
            deck.push(Card::new(Mark::Spade, to_str(&(i - 26))));
        } else {
            deck.push(Card::new(Mark::Clover, to_str(&(i - 39))));
        }
    }
    deck
}

fn to_number(number: &str) -> u8 {
    match number {
        "A" => 1,
        "J" | "Q" | "K" => 10,
        _ => number.parse().unwrap(),
    }
}

fn to_str(number: &u8) -> &str {
    match number {
        1 => "A",
        11 => "J",
        12 => "Q",
        13 => "K",
        _ => &number.to_string(),
    }
}

fn decide_card(deck: &Vec<Card>) -> Card {
    let mut rng = rand::thread_rng();
    deck[rng.gen_range(0, 52)]
}

fn remove_card(deck: &Vec<Card>, card: &Card) -> &Vec<Card> {
    for i in 0..deck.len() {
        match deck[i].mark {
            Mark::Diamond => {
                if deck[i].number == card.number {
                    let mut deck = deck.remove(i);
                    break;
                }
            },
            Mark::Heart => {
                if deck[i].number == card.number {
                    let mut deck = deck.remove(i);
                    break;
                }
            },
            Mark::Spade => {
                if deck[i].number == card.number {
                    let mut deck = deck.remove(i);
                    break;
                }
            },
            Mark::Clover => {
                if deck[i].number == card.number {
                    let mut deck = deck.remove(i);
                    break;
                }
            },
        }
    }
    deck
}

fn draw_player(card: &Card, score: u8) -> u8 {
    println!("あなたの引いたカードは{}の{}です。", card.mark, card.number);
    score + card.number
}

fn draw_dealer(card: &Card, score: u8) -> u8 {
    println!(
        "ディーラーの引いたカードは{}の{}です。",
        card.mark, card.number
    );
    score + card.number
}

fn check_winner(player_score: u8, dealer_score: u8) {
    if dealer_score > 21 || player_score > dealer_score {
        println!("あなたの勝ち！");
    } else if player_score == dealer_score {
        println!("引き分け！");
    } else {
        println!("あなたの負け…");
    }
}

fn main() {
    println!("ブラックジャックへようこそ！");
    println!("ゲームを開始します");
    let player_score: u8 = 0;
    let dealer_score: u8 = 0;
    let deck = generate_deck();

    let player_1 = decide_card(deck);
    let player_score = draw_player(&player_1, player_score);

    let player_2 = decide_card(&mark, &number);
    let mut player_score = draw_player(&player_2, player_score);

    let dealer_1 = decide_card(&mark, &number);
    let dealer_score = draw_dealer(&dealer_1, dealer_score);

    println!("ディーラーの2枚目のカードはわかりません。");

    while player_score < 21 {
        println!("あなたの現在の得点は{}です。", player_score);
        println!("カードを引きますか？引く場合はYを、引かない場合はNを入力してください");
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("読み込みに失敗しました");
        let input_str: &str = &input_str.trim();
        match input_str {
            "Y" => {
                let player_card = decide_card(&mark, &number);
                player_score = draw_player(&player_card, player_score);
                continue;
            }
            "N" => {
                break;
            }
            _ => println!("YかNを入力してください"),
        }
    }

    if player_score > 21 {
        println!("21点を超えてしまいました。あなたの負けです");
        return;
    }

    let dealer_2 = decide_card(&mark, &number);
    println!(
        "ディーラーの2枚目のカードは{}の{}でした。",
        dealer_2.mark, dealer_2.number
    );
    let mut dealer_score = dealer_score + dealer_2.number;
    println!("ディーラーのスコアは{}点です。", dealer_score);

    while dealer_score < 17 {
        let dealer_card = decide_card(&mark, &number);
        dealer_score = draw_dealer(&dealer_card, dealer_score);
        println!("ディーラーのスコアは{}点です。", dealer_score);
    }

    check_winner(player_score, dealer_score);
}
