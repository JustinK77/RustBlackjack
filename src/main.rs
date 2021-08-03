
use std::io;
use std::fmt;
use rand::Rng;
use std::cmp::Ordering;
//use std::vec;
use std::{thread, time};

struct Card{
    writable: String,
    value: u32
}

impl fmt::Display for Card{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.writable)
    }
}

//TO-DO:
//Split
//Deck logic?

fn main() {
    println!("~~~Welcome to the table!~~~");
    let mut money : f32 = 100.0;
    loop{
        if money <= 0.0{
            println!("Out of money, adding 10 dollars");
            money = 10.0;
        }
        println!("You have {}, enter bet amount: ", money);
        let mut bet_string = String::new();
        io::stdin()
            .read_line(&mut bet_string)
            .expect("Please input valid amount.");
        let mut bet : f32 = match bet_string.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        if bet > money{
            println!("You don't have that much!");
            continue;
        }
        let mut dealer_hand: Vec<Card> = Vec::new();
        dealer_hand.push(get_card());
        let mut dealer_hand_value = dealer_hand[0].value;
        println!("Dealer showing {}, total {}", dealer_hand[0].writable, dealer_hand[0].value);
        let five_hundred_millis = time::Duration::from_millis(500);
        thread::sleep(five_hundred_millis);
        let mut hand: Vec<Card> = Vec::new();
        let mut hand_val = 0;
        hand.push(get_card());
        hand.push(get_card());
        for card in &hand{
            println!("{}", card);
            thread::sleep(five_hundred_millis);
            hand_val += card.value;
        }
        if hand_val == 21{
            println!("Blackjack!");
            money += bet * 1.5;
            continue;
        }
        if hand_val == 22{
            hand_val = 12;
            hand.remove(0);
        }
        println!("Current hand value: {}", hand_val);
        while hand_val < 21{
            println!("Hit (H), Stand (S), or Double Down (D)?");
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Please input H or S.");
            let action : &str = &action[..].trim();
            match action{
                "H" | "h" => {
                    thread::sleep(five_hundred_millis);
                    let drawn_card : Card = get_card();
                    hand_val += drawn_card.value;
                    hand.push(drawn_card);
                    println!("Drew {}", hand[hand.len() - 1].writable);
                    if hand_val > 21 && has_ace(&hand){
                        hand_val -= 10;
                        hand.clear();
                    }
                    println!("Current hand value: {}", hand_val);
                }
                "D" | "d" => {
                    bet *= 1.5;
                    println!("New bet: {}", bet);     
                    thread::sleep(five_hundred_millis);
                    let drawn_card : Card = get_card();
                    hand_val += drawn_card.value;
                    hand.push(drawn_card);
                    println!("Drew {}", hand[hand.len() - 1].writable);
                    if hand_val > 21 && has_ace(&hand){
                        hand_val -= 10;
                        hand.clear();
                    }
                    println!("Current hand value: {}", hand_val);
                    break;
                }
                "S" | "s" => {
                    println!("Standing at {}", hand_val);
                    thread::sleep(five_hundred_millis);
                    break;
                }
                _ => break
            }
        }
        if hand_val > 21{
            println!("Bust!");
            money -= bet;
        }
        else{
            while dealer_hand_value < 17{
                dealer_hand.push(get_card());
                dealer_hand_value += dealer_hand[dealer_hand.len() - 1].value;
                print!("Dealer draws {}, ", dealer_hand[dealer_hand.len() - 1].writable);
                if dealer_hand_value > 21 && has_ace(&dealer_hand){
                    dealer_hand_value -= 10;
                    dealer_hand.clear();
                }
                println!("total {}", dealer_hand_value);
                thread::sleep(five_hundred_millis);
            }
            if dealer_hand_value > 21{
                println!("Dealer busts, you win!");
                money += bet;
            }
            else{
                match hand_val.cmp(&dealer_hand_value){
                    Ordering::Less => {
                        println!("Lose!");
                        money -= bet;
                    }
                    Ordering::Equal => println!("Push!"),
                    Ordering::Greater => {
                        println!("Win!");
                        money += bet;
                    }
                }
            }
        }
        println!();
        println!("~~~NEW HAND~~~");
    }
}

fn get_card() -> Card{
    let mut value = rand::thread_rng().gen_range(1..=13);
    let suit = rand::thread_rng().gen_range(1..=4);
    let mut writable = String::new();
    match value{
        1 => writable.push_str("Ace of "),
        2 => writable.push_str("Two of "),
        3 => writable.push_str("Three of "),
        4 => writable.push_str("Four of "),
        5 => writable.push_str("Five of "),
        6 => writable.push_str("Six of "),
        7 => writable.push_str("Seven of "),
        8 => writable.push_str("Eight of "),
        9 => writable.push_str("Nine of "),
        10 => writable.push_str("Ten of "),
        11 => writable.push_str("Jack of "),
        12 => writable.push_str("Queen of "),
        13 => writable.push_str("King of "),
        _ => writable.push_str("INVALID")
    }
    match suit{
        1 => writable.push_str("Diamonds"),
        2 => writable.push_str("Hearts"),
        3 => writable.push_str("Spades"),
        4 => writable.push_str("Clubs"),
        _ => writable.push_str("INVALID")
    }
    if value > 10{
        value = 10;
    }
    if value == 1{
        value = 11;
    }
    Card{
        writable: writable,
        value: value
    }
}

fn has_ace(hand : &Vec<Card>) -> bool{
    for card in hand{
        if card.value == 11{
            return true;
        }
    }
    false
}
