// this should be a rock-paper-scissors game

use std::io::{BufRead, Stdin};

use rand::{rngs::ThreadRng, Rng};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn get_user_move(stdin: &Stdin) -> Move {
    // accept r or rock, p or paper, s or scissors
    let mut user_input = String::new();

    println!("Enter your move (r, p, or s):");
    stdin
        .lock()
        .read_line(&mut user_input)
        .expect("failed to read user input");

    match user_input.to_ascii_lowercase().trim() {
        "r" | "rock" => Move::Rock,
        "p" | "paper" => Move::Paper,
        "s" | "scissors" => Move::Scissors,
        _ => {
            println!("invalid move. please enter r, p, or s.");
            get_user_move(stdin)
        }
    }
}

fn get_computer_move(rng: &mut ThreadRng) -> Move {
    // randomly select rock, paper, or scissors
    let random_number = rng.gen_range(0..=2);
    match random_number {
        0 => Move::Rock,
        1 => Move::Paper,
        2 => Move::Scissors,
        _ => unreachable!(),
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rng = rand::thread_rng();
    let mut current_round = 0;

    println!("starting a rock-paper-scissors game!");

    loop {
        println!("round {}", current_round);

        let user_move = get_user_move(&stdin);
        let computer_move = get_computer_move(&mut rng);

        println!("you chose: {:?}", user_move);
        println!("computer chose: {:?}", computer_move);

        match (user_move, computer_move) {
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => println!("you win!"),
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => println!("computer wins!"),
            _ => println!("it's a tie!"),
        }

        current_round += 1;
    }
}
