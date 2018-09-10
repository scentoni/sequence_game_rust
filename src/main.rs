extern crate ncurses;
extern crate rand;

use std::env;
use rand::Rng;

macro_rules! writeln {
    ($fmt:expr) => (write!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (write!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! write {
    ($fmt:expr) => (ncurses::printw($fmt); ncurses::refresh(););
    ($fmt:expr, $($arg:tt)*) => (ncurses::printw(&format!($fmt, $($arg)*)); ncurses::refresh(););
}

fn getnumber(prompt: &str, max: usize) -> usize {
    write!("{}", prompt);
    loop {
        let ch1 = ncurses::getch();
        let ch2 = std::char::from_u32(ch1 as u32);
        if let Some(c) = ch2 {
            if let Some(nu32) = c.to_digit(10) {
                let n = nu32 as usize;
                if n <= max {
                    return n
                }
            }
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
        writeln!("{}", sequence_display.join(" "));
        writeln!("{}", labels.join(" "));
    }

    fn is_sorted(&self) -> bool {
        return Game::sorted_length(&self.sequence) >= self.sequence.len();
    }
}

fn game(length: usize) {
    let mut turn = 0;
    ncurses::initscr();
    let mut game = Game::new_random(length);
    game.print();
    while !game.is_sorted() {
        turn += 1;
        writeln!("Turn {}", turn);
        let left = getnumber("Reverse from:", length as usize);
        writeln!("");
        let right = getnumber("Reverse through:", length as usize);
        game.reverse_segment(left, right);
        writeln!("");
        game.print();
    }
    ncurses::refresh();
    ncurses::endwin();
    println!("You solved the puzzle in {} moves!", turn);
}

fn print_usage(program: &str) {
    println!("Usage: {} <length>", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 2 {
        println!("{} is the wrong number of arguments", args.len() - 1);
        print_usage(&program);
    } else {
        match args[1].trim().parse() {
            Ok(len) => {
                if 1 <= len && len <= 10 {
                    game(len);
                } else {
                    println!("{} is not in the range [1..10]", len);
                    print_usage(&program);
                }
            },
            Err(_) => {
                println!("Could not interpret \"{}\" as an integer", args[1]);
                print_usage(&program);
            },
        };
    };
}
