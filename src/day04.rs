use std::fmt::{self, write, format};
use std::fmt::Write;

use array2d::Array2D;
use itertools::Itertools;


pub fn part1(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>();
    parse_input(lines);
    
}

pub fn part2(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>();

}

#[derive(Debug)]
struct BingoBoard {
    label: usize,
    board: Array2D<(bool, usize)> 
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!("label: {}", self.label);
        let mut rows = String::new();
        for row in self.board.rows_iter() {
            write!(&mut rows, "{:?}\n", row.collect::<Vec<&(bool, usize)>>());
        };
        write!(f, "{}\n{}", label, rows)
    }
}

fn parse_input(lines: Vec<&str>) {
    let mut lines = lines.iter();
    let callout_numbers = lines.next().expect("Failed to get the callout numbers.");
    let mut label = 0;
    let mut boards: Vec<BingoBoard> = vec![];
    lines.next(); // skip the first newline after callouts
    let mut rows = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            boards.push(
                BingoBoard {
                    label,
                    board: Array2D::from_rows(&rows)
                }
            );
            rows.clear();
            label += 1;
            continue;
        }
        let row: Vec<(bool, usize)> = line.split_whitespace().map(|x| {
            (false, x.parse::<usize>().unwrap())
        }).collect();
        rows.push(row);
    }
    boards.push(
        BingoBoard {
            label,
            board: Array2D::from_rows(&rows)
        }
    );
    for board in boards {
        println!("{}", &board);
    }
}