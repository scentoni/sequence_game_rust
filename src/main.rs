extern crate rand;

use std::env;
use std::io;
use rand::Rng;

fn getnumber(prompt: &str, max: usize) -> usize {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let num: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if num <= max {
            return num
        }
    }
}

#[derive(Debug)]
struct Game {
    sequence: Vec<usize>,
}

impl Game {
    fn new_random(length: usize) -> Game {
        let mut sequence: Vec<_> = (0..length).collect();
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut sequence);
        Game {
            sequence,
        }
    }

    fn sorted_length<T: PartialOrd>(sequence: &Vec<T>) -> usize {
        let mut i = 1;
        while i < sequence.len() && sequence[i - 1] < sequence[i] {
            i += 1;
        }
        return i;
    }

    fn reverse_segment(&mut self, left: usize, right: usize) {
        self.sequence[left..(right + 1)].reverse();
    }

    fn print(&self) {
        static ASCII_LOWER: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
            'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let sequence_display: Vec<_> = self.sequence.iter().map(|i| ASCII_LOWER[*i].to_string() ).collect();
        let labels: Vec<_> = (0..self.sequence.len()).map(|i| (i).to_string() ).collect();
        println!("{}", sequence_display.join(" "));
        println!("{}", labels.join(" "));
    }

    fn is_sorted(&self) -> bool {
        return Game::sorted_length(&self.sequence) >= self.sequence.len();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length: usize = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => 5,
    };

    let mut turn = 0;
    let mut game = Game::new_random(length);

    game.print();
    while !game.is_sorted() {
        turn += 1;
        println!("Turn {}", turn);
        let left = getnumber("Reverse from:", length as usize);
        let right = getnumber("Reverse through:", length as usize);
        game.reverse_segment(left, right);
        println!();
        game.print();
    }
    println!("You won in {} turns!", turn);
}
