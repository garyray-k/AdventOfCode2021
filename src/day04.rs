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

struct BingoBoard {
    label: usize,
    board: Array2D<(bool, usize)> 
}

fn parse_input(lines: Vec<&str>) {
    let mut lines = lines.iter();
    let callout_numbers = lines.next().expect("Failed to get the callout numbers.");
    let mut label = 0;
    let mut row_counter = 0;
    let boards: Vec<BingoBoard> = vec![];
    lines.next(); // skip the first newline after callouts
    while let Some(line) = lines.next() {
        if line.is_empty() {
            row_counter = 0;
            label += 1;
            println!("-------------");
            continue;
        }
        let row: Vec<(bool, usize)> = line.split_whitespace().map(|x| {
            (false, x.parse::<usize>().unwrap())
        }).collect();
        println!("{:?}", row);
    }
}