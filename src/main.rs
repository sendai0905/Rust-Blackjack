extern crate rand;

// use std::io;
use rand::Rng;

fn to_number(number: &str) -> u8 {
    match number {
        "A" => 1,
        "J" | "Q" | "K" => 10,
        _ => number.parse().unwrap(),
    }
}

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

fn deal_card(mark: &Vec<&str>, number: &Vec<&str>) -> Card {
    let mut rng = rand::thread_rng();
    let card_mark = mark[rng.gen_range(0, 4)];
    let card_score = number[rng.gen_range(0, 13)];
    Card::new(card_mark, card_score)
}

fn main() {
    println!("ブラックジャックへようこそ！");
    println!("ゲームを開始します");
    let mark = vec!["ダイヤ", "ハート", "スペード", "クローバー"];
    let number = vec![
        "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];
    let mut player_score: u8 = 0;
    // let mut dealer_score: u8 = 0;

    let player_1 = deal_card(&mark, &number);
    println!(
        "あなたの引いたカードは{}の{}です。",
        player_1.mark, player_1.number
    );
    player_score += player_1.number;

    let player_2 = deal_card(&mark, &number);
    println!(
        "あなたの引いたカードは{}の{}です。",
        player_2.mark, player_2.number
    );
    player_score += player_2.number;

    let dealer_1 = deal_card(&mark, &number);
    println!(
        "ディーラーの引いたカードは{}の{}です。",
        dealer_1.mark, dealer_1.number
    );
    // dealer_score += dealer_1.number;
    println!("ディーラーの2枚目のカードはわかりません。");

    println!("あなたの現在の得点は{}です。", player_score);
    println!(
        "カードを引きますか？引く場合はYを、引かない場合はNを入力してください"
    );
    // let mut input_str = String::new();
    // match io::stdin().read_line(&mut input_str) {
    //     "Y" => {},
    //     "N" => {},
    //     _ =>
    // }
}
