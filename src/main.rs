extern crate rand;

use rand::Rng;
use std::io;

struct Card {
    mark: String,
    number: u8,
}

impl Card {
    fn new(mark: &str, number: &str) -> Card {
        Card {
            mark: mark.to_string(),
            number: to_number(number),
        }
    }
}

fn to_number(number: &str) -> u8 {
    match number {
        "A" => 1,
        "J" | "Q" | "K" => 10,
        _ => number.parse().unwrap(),
    }
}

fn decide_card(mark: &Vec<&str>, number: &Vec<&str>) -> Card {
    let mut rng = rand::thread_rng();
    let card_mark = mark[rng.gen_range(0, 4)];
    let card_score = number[rng.gen_range(0, 13)];
    Card::new(card_mark, card_score)
}

fn draw_player(card: &Card, score: u8) -> u8 {
    println!(
        "あなたの引いたカードは{}の{}です。",
        card.mark, card.number
    );
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
    let mark = vec!["ダイヤ", "ハート", "スペード", "クローバー"];
    let number = vec![
        "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];
    let player_score: u8 = 0;
    let dealer_score: u8 = 0;

    let player_1 = decide_card(&mark, &number);
    let player_score = draw_player(&player_1, player_score);

    let player_2 = decide_card(&mark, &number);
    let mut player_score = draw_player(&player_2, player_score);

    let dealer_1 = decide_card(&mark, &number);
    let dealer_score = draw_dealer(&dealer_1, dealer_score);

    println!("ディーラーの2枚目のカードはわかりません。");

    while player_score < 21 {
        println!("あなたの現在の得点は{}です。", player_score);
        println!(
            "カードを引きますか？引く場合はYを、引かない場合はNを入力してください"
        );
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
