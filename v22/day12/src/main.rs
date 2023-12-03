use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::Iterator;

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const HEIGHT: usize = 40;
const WIDTH: usize = 159;

fn find_char(input: &str, target_ch: &char) -> Option<Coordinate> {
    {
        for (y_index, line) in input.lines().enumerate() {
            for (x_index, ch) in line.chars().enumerate() {
                if &ch == target_ch {
                    println!("{} found at {}/{}", target_ch, y_index, x_index);
                    return Some(Coordinate {
                        y: y_index,
                        x: x_index,
                    });
                }
            }
        }
    }
    None
}
#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn choices(current: &Coordinate, end: &Coordinate) -> [Direction; 4] {
        if current.y < end.y && current.x < end.x {
            // We are too low, and too far left -> prefer up and right
            [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
        } else if current.y < end.y && current.x >= end.x {
            // We are too low, and too far right -> prefer up and left
            [
                Direction::Up,
                Direction::Left,
                Direction::Down,
                Direction::Right,
            ]
        } else if current.y > end.y && current.x >= end.x {
            // We are too high, and too far right -> prefer down and left
            [
                Direction::Down,
                Direction::Left,
                Direction::Up,
                Direction::Right,
            ]
        } else {
            // We are too high, and too far left -> prefer down and right
            [
                Direction::Down,
                Direction::Right,
                Direction::Left,
                Direction::Up,
            ]
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn turn_to(&self, direction: Direction) -> Option<Coordinate> {
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    None
                } else {
                    Some(Coordinate {
                        y: self.y - 1,
                        x: self.x,
                    })
                }
            }
            Direction::Down => {
                if self.y == HEIGHT - 1 {
                    None
                } else {
                    Some(Coordinate {
                        y: self.y + 1,
                        x: self.x,
                    })
                }
            }
            Direction::Right => {
                if self.x == WIDTH - 1 {
                    None
                } else {
                    Some(Coordinate {
                        x: self.x + 1,
                        y: self.y,
                    })
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    None
                } else {
                    Some(Coordinate {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
        }
    }
    fn char(&self, input: &str) -> char {
        input
            .lines()
            .nth(self.y)
            .unwrap()
            .chars()
            .nth(self.x)
            .unwrap()
    }
}

fn is_move_legal(current_char: &char, input: &str, option: &Coordinate) -> bool {
    let opt_char = option.char(input);

    if current_char == &'S' || opt_char == 'E' {
        return true;
    };

    let d: i8 = ALPHABET.iter().position(|ch| ch == current_char).unwrap() as i8
        - ALPHABET.iter().position(|ch| ch == &opt_char).unwrap() as i8;

    if -1 > d {
        false
    } else {
        d <= 1
    }
}

fn get_move(
    last: &Coordinate,
    end: &Coordinate,
    tried_moves: &HashMap<Move, bool>,
    input: &str,
    visited_coordinates: &Vec<Coordinate>,
) -> Option<Move> {
    for direction in Direction::choices(&last, &end) {
        // println!("{:?}", direction);
        // Check if the move is on the board
        if let Some(possible_move) = last.turn_to(direction) {
            // println!("move? {:?}", possible_move);
            // Check if the move has been exhausted already
            let mv = Move {
                from: *last,
                to: possible_move,
            };
            let last_char = last.char(input);
            if let Some(v) = tried_moves.get(&mv) {
                if *v {
                    // Allow moving this direction if true
                    if !visited_coordinates.contains(&possible_move)
                        && is_move_legal(&last_char, input, &possible_move)
                    {
                        return Some(mv);
                    } else {
                        continue;
                    }
                } else {
                    // Block otherwise. If false, we've already
                    // established this path is bad.
                    continue;
                }
            }
            // If the path hasn't been tried, try it
            if !visited_coordinates.contains(&possible_move)
                && is_move_legal(&last_char, input, &possible_move)
            {
                return Some(mv);
            } else {
                continue;
            }
        }
    }
    None
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Move {
    from: Coordinate,
    to: Coordinate,
}

fn main() {
    // Max iterations before returning. I think this could run for a long time otherwise.
    let iterations = 100000000;

    // Read file
    let input = fs::read_to_string("day12/src/input").unwrap();

    // Find end coordinate
    let end = find_char(&input, &'E').unwrap();

    // Create mutable variable for the last coordinate
    let mut last = Coordinate { x: 0, y: 0 };

    // Keep track of moves we've tried that didn't work
    let mut tried_moves: HashMap<Move, bool> = HashMap::new();

    // Keep track of the coordinates we've already visited in the current run
    let mut visited_coordinates = Vec::new();

    // Mutable variable for the step of each try
    let mut steps = 0;

    // Fewest steps found
    let mut fewest: u64 = 999999999999999;

    for _ in 0..iterations {
        // Get new move
        let current_move = match get_move(&last, &end, &tried_moves, &input, &visited_coordinates) {
            Some(t) => t,
            None => {
                // Bad path, go back
                // println!("No moves to make at {:?}", last);
                tried_moves
                    .insert(
                        Move {
                            from: last,
                            to: last,
                        },
                        false,
                    )
                    .unwrap_or(true);
                visited_coordinates.clear();
                last = Coordinate { x: 0, y: 0 };
                steps = 0;
                continue;
            }
        };
        steps += 1;

        visited_coordinates.push(current_move.to);
        tried_moves.insert(current_move, true); // is this useful?
        last = current_move.to;

        // if we get to end, calculate fewest and start again
        if current_move.to.char(&input) == 'E' {
            if steps < fewest {
                println!("Found end in {} steps: {:?}", steps, visited_coordinates);
                for coord in &visited_coordinates {
                    print!("--> {}", coord.char(&input));
                }
                fewest = steps;
            }
            last = Coordinate { x: 0, y: 0 };
            visited_coordinates.clear();
            steps = 0;
            continue;
        }
    }

    println!("\n{}", fewest);
}
