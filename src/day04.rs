use std::fmt::Write;
use std::fmt::{self, format, write};
use std::process;

use array2d::Array2D;
use itertools::Itertools;

pub fn part1(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>();
    let (mut boards, values) = parse_input(lines);
    for value in values {
        let value: usize = value.parse().unwrap();
        boards = mark_board(&mut boards, value);
        if let Some(winner) = find_winner(&boards) {
            let bingo = boards
                .iter()
                .find_or_first(|x| x.label == winner)
                .expect("No winner found.");
            let unmarked_sum = bingo
                .board
                .elements_row_major_iter()
                .filter(|x| !x.0)
                .fold(0, |a: usize, b| a + b.1);
            println!("{}", bingo.label);
            println!("Answer: {}", unmarked_sum * value);
            break;
        }
    }
}

pub fn part2(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>();
    let (mut boards, values) = parse_input_winner(lines);
    let mut winners: Vec<BingoBoardWinner> = vec![];
    for value in values {
        boards = mark_board_winner(&mut boards, value.parse().unwrap());
        boards.retain(|x| {
            if x.winner.is_some() {
                winners.push(x.clone());
            }
            x.winner.is_none()
        })
    }
    let last_winner = winners.last().unwrap();
    let unmakred_sum = last_winner
        .board
        .elements_row_major_iter()
        .filter(|x| !x.0)
        .fold(0, |a: usize, b| a + b.1);
    println!("{:?}", last_winner.winner.unwrap() * unmakred_sum);
}

#[derive(Debug, Clone)]
struct BingoBoard {
    label: usize,
    board: Array2D<(bool, usize)>,
}
#[derive(Debug, Clone)]
struct BingoBoardWinner {
    label: usize,
    board: Array2D<(bool, usize)>,
    winner: Option<usize>,
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!("label: {}", self.label);
        let mut rows = String::new();
        for row in self.board.rows_iter() {
            write!(&mut rows, "{:?}\n", row.collect::<Vec<&(bool, usize)>>());
        }
        write!(f, "{}\n{}", label, rows)
    }
}

fn parse_input(lines: Vec<&str>) -> (Vec<BingoBoard>, Vec<&str>) {
    let mut lines = lines.iter();
    let callout_numbers = lines
        .next()
        .expect("Failed to get the callout numbers.")
        .split(',')
        .collect::<Vec<&str>>();
    let mut label = 0;
    let mut boards: Vec<BingoBoard> = vec![];
    lines.next(); // skip the first newline after callouts
    let mut rows = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            boards.push(BingoBoard {
                label,
                board: Array2D::from_rows(&rows),
            });
            rows.clear();
            label += 1;
            continue;
        }
        let row: Vec<(bool, usize)> = line
            .split_whitespace()
            .map(|x| (false, x.parse::<usize>().unwrap()))
            .collect();
        rows.push(row);
    }
    boards.push(BingoBoard {
        label,
        board: Array2D::from_rows(&rows),
    });
    (boards, callout_numbers)
}

fn parse_input_winner(lines: Vec<&str>) -> (Vec<BingoBoardWinner>, Vec<&str>) {
    let mut lines = lines.iter();
    let callout_numbers = lines
        .next()
        .expect("Failed to get the callout numbers.")
        .split(',')
        .collect::<Vec<&str>>();
    let mut label = 0;
    let mut boards: Vec<BingoBoardWinner> = vec![];
    lines.next(); // skip the first newline after callouts
    let mut rows = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            boards.push(BingoBoardWinner {
                label,
                board: Array2D::from_rows(&rows),
                winner: None,
            });
            rows.clear();
            label += 1;
            continue;
        }
        let row: Vec<(bool, usize)> = line
            .split_whitespace()
            .map(|x| (false, x.parse::<usize>().unwrap()))
            .collect();
        rows.push(row);
    }
    boards.push(BingoBoardWinner {
        label,
        board: Array2D::from_rows(&rows),
        winner: None,
    });
    (boards, callout_numbers)
}

fn mark_board(boards: &mut Vec<BingoBoard>, value: usize) -> Vec<BingoBoard> {
    for bingo in boards.iter_mut() {
        for row in 0..5 {
            for column in 0..5 {
                let square = bingo
                    .board
                    .get(row, column)
                    .expect("Failed to get bing square")
                    .clone();
                if square.1 == value {
                    bingo
                        .board
                        .set(row, column, (true, square.1))
                        .expect("Failed to set value");
                }
            }
        }
    }
    boards.to_owned()
}

fn mark_board_winner(boards: &mut Vec<BingoBoardWinner>, value: usize) -> Vec<BingoBoardWinner> {
    for bingo in boards.iter_mut() {
        for row in 0..5 {
            for column in 0..5 {
                let square = bingo
                    .board
                    .get(row, column)
                    .expect("Failed to get bing square")
                    .clone();
                if square.1 == value {
                    bingo
                        .board
                        .set(row, column, (true, square.1))
                        .expect("Failed to set value");
                }
            }
        }
        for mut row in bingo.board.rows_iter() {
            let winner = row.all(|x| x.0);
            if winner {
                bingo.winner = Some(value);
            }
        }
        for mut column in bingo.board.columns_iter() {
            let winner = column.all(|x| x.0);
            if winner {
                bingo.winner = Some(value);
            }
        }
    }
    boards.to_owned()
}

fn find_winner(boards: &Vec<BingoBoard>) -> Option<usize> {
    for bingo in boards {
        for mut row in bingo.board.rows_iter() {
            let winner = row.all(|x| x.0);
            if winner {
                return Some(bingo.label);
            }
        }
        for mut column in bingo.board.columns_iter() {
            let winner = column.all(|x| x.0);
            if winner {
                return Some(bingo.label);
            }
        }
    }
    None
}
