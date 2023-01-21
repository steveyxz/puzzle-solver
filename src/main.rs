use std::io::stdin;

use fire_circle::solver::{self, FireCircle};

mod fire_circle;

fn main() {
    fire_circle_solver();
}

fn fire_circle_solver() {
    println!("Enter starting board state (O is unlit, # is lit): ");
    let mut starting = String::new();
    stdin()
        .read_line(&mut starting)
        .expect("Error while reading input");
    if starting.chars().all(|c| c == 'O' || c == '#') {
        panic!("Invalid input, must only contain O or #!");
    }
    let mut starting_board = Vec::new();

    for character in starting.trim().chars() {
        if character == '#' {
            starting_board.push(true);
        } else {
            starting_board.push(false);
        }
    }

    println!("Enter maximum depth: ");

    let mut depth_input = String::new();
    stdin()
        .read_line(&mut depth_input)
        .expect("Error while reading input!");
    let depth: i32 = depth_input.trim().parse::<i32>().unwrap();

    let solution = solver::solve(FireCircle(starting_board), depth);
    println!("{:?}", solution.unwrap());
}
