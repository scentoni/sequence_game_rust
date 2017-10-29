extern crate rand;

use std::env;
use std::io;
use rand::Rng;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn getnumber(max: usize) -> usize {
    let mut input = String::new();
    loop {
        println!("Rotate through: ");
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

fn print_sequence(sequence: &Vec<usize>) {
    let sequence_display: Vec<_> = sequence.iter().map(|i| ASCII_LOWER[*i].to_string() ).collect();
    let labels: Vec<_> = (0..sequence.len()).map(|i| (i).to_string() ).collect();
    println!("{}", sequence_display.join(" "));
    println!("{}", labels.join(" "));
}

fn sorted_length<T: PartialOrd>(sequence: &Vec<T>) -> usize {
    let mut i = 1;
    while i < sequence.len() && sequence[i - 1] < sequence[i] {
        i += 1;
    }
    return i;
}

fn reverse_segment<T>(sequence: &mut Vec<T>, left: usize, right: usize) {
    sequence[left..(right + 1)].reverse();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length: usize = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => 5,
    };

    let mut turn = 0;
    let mut sequence: Vec<_> = (0..length).collect();
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut sequence);
    sequence.reverse();

    print_sequence(&sequence);
    while sorted_length(&sequence) < sequence.len() {
        turn += 1;
        println!("Turn {}", turn);
        let right = getnumber(length as usize);
        let left = 0;
        reverse_segment(&mut sequence, left, right);
        println!();
        print_sequence(&sequence);
    }
    println!("You won in {} turns!", turn);
}
